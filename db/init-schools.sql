CREATE TABLE IF NOT EXISTS schools (
    id uuid PRIMARY KEY,
    school_id text NOT NULL UNIQUE,
    name text NOT NULL,
    description text NOT NULL
);

INSERT INTO schools (id, school_id, name, description)
SELECT *
FROM (
    VALUES
        (
            uuidv7(),
            'stoicism',
            'Stoicism',
            $$Stoicism is a philosophical movement and practical guide to living, emphasizing daily self-discipline and moral improvement, which originated in the Hellenistic period of ancient Greece and proliferated well into the Roman Imperial period.

The ancient Stoics believed that the universe operated according to reason, or logos, providing a unified account of the world, constructed from ideals of rational discourse, monistic physics, and naturalistic ethics.

These ideals constitute virtue, which is necessary for the Stoic goal of 'living a well-reasoned life'.$$
        ),
        (
            uuidv7(),
            'platonism',
            'Platonism',
            $$Platonism is the philosophy of Plato and philosophical systems closely derived from it, considered the opposite of nominalism, or anti-realism.

Platonism has had a profound influence on Western thought. Platonism or Platonic realism affirms the real existence of forms or abstract objects, originally to solve the problem of universals.

Abstract objects are asserted to exist in a third realm distinct from both the sensible external world and from the internal world of consciousness. This can apply to properties, types, propositions, meanings, numbers, sets, truth values, and so on (see abstract object theory).$$
        ),
        (
            uuidv7(),
            'peripatetic',
            'Peripatetic school',
            $$The Peripatetic school (Ancient Greek: 'walkway') was a philosophical school founded in 335 BC by Aristotle in the Lyceum in ancient Athens.

It was an informal institution whose members conducted philosophical and scientific inquiries. The school fell into decline after the middle of the 3rd century BC, but had a revival in the Roman Empire.$$
        )
) AS v(id, school_id, name, description)
WHERE NOT EXISTS (
    SELECT 1 FROM schools
);