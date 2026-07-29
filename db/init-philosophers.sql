CREATE TABLE IF NOT EXISTS philosophers (
    id uuid PRIMARY KEY,
    philosopher_id text NOT NULL UNIQUE,
    name text NOT NULL,
    description text NOT NULL
);

INSERT INTO philosophers (id, philosopher_id, name, description)
SELECT *
FROM (
    VALUES
        (
            uuidv7(),
            'marcus-aurelius',
            'Marcus Aurelius',
            $$Marcus Aurelius Antoninus (26 April 121 – 17 March 180) was Roman emperor from 161 to 180 and a Stoic philosopher.

He was a member of the Nerva–Antonine dynasty, the last of the rulers later known as the Five Good Emperors and the last emperor of the Pax Romana, an age of relative peace, calm, and stability for the Roman Empire lasting from 27 BC to 180 AD.

He served as Roman consul in 140, 145, and 161.$$
        ),
        (
            uuidv7(),
            'heraclitus',
            'Heraclitus',
            $$Heraclitus (fl. c. 500 BC) was a pre-Socratic Greek philosopher from the city of Ephesus, which was then part of the Persian Empire.

He exerts a wide influence on Western philosophy, both ancient and modern, through the works of such authors as Plato, Aristotle, the Stoics, Georg Wilhelm Friedrich Hegel, Friedrich Nietzsche, and Martin Heidegger.$$
        ),
        (
            uuidv7(),
            'epictetus',
            'Epictetus',
            $$Epictetus (c. 50 – c. 135 AD) was a Greek Stoic philosopher. He was born into slavery at Hierapolis, Phrygia (present-day Pamukkale, in western Turkey) and lived in Rome until his banishment, after which he spent the rest of his life in Nicopolis in northwestern Greece.

Epictetus studied Stoic philosophy under Musonius Rufus and after manumission, his formal emancipation from slavery, he began to teach philosophy.$$
        )
) AS v(id, philosopher_id, name, description)
WHERE NOT EXISTS (
    SELECT 1 FROM philosophers
);