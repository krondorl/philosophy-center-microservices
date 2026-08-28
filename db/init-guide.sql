CREATE TABLE IF NOT EXISTS guides (
    id uuid PRIMARY KEY DEFAULT uuidv7(),
    slug text NOT NULL UNIQUE,
    title text NOT NULL,
    school_id text NOT NULL
);

CREATE TABLE IF NOT EXISTS guide_philosophers (
    id uuid PRIMARY KEY DEFAULT uuidv7(),
    guide_id uuid NOT NULL,
    philosopher_id text NOT NULL,

    CONSTRAINT fk_guide
        FOREIGN KEY (guide_id)
        REFERENCES guides(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_guide_philosopher
        UNIQUE (guide_id, philosopher_id)
);

INSERT INTO guides (
    slug,
    title,
    school_id
)
VALUES
    ('intro-to-stoicism', 'Introduction to Stoicism', 'stoicism'),
    ('intro-to-epicureanism', 'Introduction to Epicureanism', 'epicureanism'),
    ('intro-to-platonism', 'Introduction to Platonism', 'platonism'),
    ('intro-to-cynicism', 'Introduction to Cynicism', 'cynicism')
ON CONFLICT (slug) DO NOTHING;

INSERT INTO guide_philosophers (
    guide_id,
    philosopher_id
)
SELECT
    g.id,
    p.philosopher_id
FROM guides g
JOIN (
    VALUES
        ('intro-to-stoicism', 'epictetus'),
        ('intro-to-stoicism', 'seneca'),
        ('intro-to-stoicism', 'marcus-aurelius'),

        ('intro-to-epicureanism', 'epicurus'),
        ('intro-to-epicureanism', 'lucretius'),
        ('intro-to-epicureanism', 'metrodorus'),

        ('intro-to-platonism', 'plato'),
        ('intro-to-platonism', 'plotinus'),
        ('intro-to-platonism', 'porphyry'),

        ('intro-to-cynicism', 'diogenes'),
        ('intro-to-cynicism', 'antisthenes'),
        ('intro-to-cynicism', 'crates')
) AS p(guide_slug, philosopher_id)
    ON g.slug = p.guide_slug
ON CONFLICT (guide_id, philosopher_id) DO NOTHING;