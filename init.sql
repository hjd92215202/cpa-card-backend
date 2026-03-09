-- 开启扩展（用于支持模糊搜索优化）
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- ==========================================
-- 1. 学科表 (Subjects)
-- ==========================================
CREATE TABLE subjects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,          -- 如：会计、税法、经济法
    description TEXT,                          -- 学科简介
    theme_color VARCHAR(20) DEFAULT '#1890ff', -- UI主题色（前端动态渲染）
    icon_url TEXT,                             -- 学科图标
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ==========================================
-- 2. 知识体系/章节表 (Categories)
-- ==========================================
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    subject_id INTEGER REFERENCES subjects(id) ON DELETE CASCADE,
    parent_id INTEGER REFERENCES categories(id) ON DELETE CASCADE, -- 支持无限级树形目录
    name VARCHAR(100) NOT NULL,
    sort_order INTEGER DEFAULT 0,              -- 排序，保证目录按大纲排列
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ==========================================
-- 3. 知识卡片表 (Cards)
-- ==========================================
CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    
    -- 【正面内容】
    title TEXT NOT NULL,                       -- 卡片标题
    essence TEXT,                              -- 知识精华 (支持 Markdown)
    difficulty SMALLINT DEFAULT 3 CHECK (difficulty BETWEEN 1 AND 5), -- 难度 1-5
    importance CHAR(1) DEFAULT 'B' CHECK (importance IN ('A', 'B', 'C')), -- 重要度 A/B/C
    
    -- 【背面内容】
    insights TEXT,                             -- 备考心得/坑点 (支持 Markdown)
    
    -- 【复习逻辑】
    interval_days INTEGER DEFAULT 1,           -- 用户设置的复习间隔（天）
    next_review_date DATE DEFAULT CURRENT_DATE, -- 下次复习日期
    last_review_date DATE,                     -- 上次复习日期
    review_count INTEGER DEFAULT 0,            -- 累计复习次数
    
    -- 【扩展属性】
    tags TEXT[],                               -- 标签数组（如 #长投 #分录）
    is_favorite BOOLEAN DEFAULT FALSE,         -- 是否收藏
    
    -- 【搜索索引字段】
    -- 自动生成全文搜索向量，包含标题、精华和心得
    search_vector tsvector GENERATED ALWAYS AS (
        to_tsvector('simple', title || ' ' || coalesce(essence, '') || ' ' || coalesce(insights, ''))
    ) STORED,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ==========================================
-- 4. 复习日志表 (Review Logs - 用于统计学习进度)
-- ==========================================
CREATE TABLE review_logs (
    id SERIAL PRIMARY KEY,
    card_id INTEGER REFERENCES cards(id) ON DELETE CASCADE,
    reviewed_at TIMESTAMPTZ DEFAULT NOW(),
    actual_interval INTEGER,                   -- 当时记录的间隔时间
    quality SMALLINT                           -- 复习质量评价（预留）
);

-- ==========================================
-- 5. 索引优化 (Indexes)
-- ==========================================

-- 全文搜索 GIN 索引
CREATE INDEX idx_cards_search ON cards USING GIN(search_vector);

-- 常用查询优化索引
CREATE INDEX idx_cards_next_review ON cards(next_review_date);
CREATE INDEX idx_cards_category ON cards(category_id);
CREATE INDEX idx_categories_parent ON categories(parent_id);

-- 触发器：自动更新 updated_at 时间
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_cards_modtime BEFORE UPDATE ON cards FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
CREATE TRIGGER update_subjects_modtime BEFORE UPDATE ON subjects FOR EACH ROW EXECUTE PROCEDURE update_modified_column();