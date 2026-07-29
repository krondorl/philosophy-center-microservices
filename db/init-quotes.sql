CREATE TABLE IF NOT EXISTS quotes (
    id uuid PRIMARY KEY,
    philosopher_id text NOT NULL,
    quote_text text NOT NULL
);

INSERT INTO quotes (id, philosopher_id, quote_text)
SELECT *
FROM (
    VALUES
        (uuidv7(), 'marcus-aurelius', 'Begin the morning by saying to thyself, I shall meet with the busybody, the ungrateful, arrogant, deceitful, envious, unsocial. All these things happen to them by reason of their ignorance of what is good and evil.'),
        (uuidv7(), 'marcus-aurelius', 'Remember that all is opinion.'),
        (uuidv7(), 'marcus-aurelius', 'Body, soul, intelligence: to the body belong sensations, to the soul appetites, to the intelligence principles.'),
        (uuidv7(), 'marcus-aurelius', 'Let no act be done without a purpose, nor otherwise than according to the perfect principles of art.'),
        (uuidv7(), 'heraclitus', 'Though the logos is common, the many live as if they had a wisdom of their own.'),
        (uuidv7(), 'heraclitus', 'If happiness consisted in the pleasures of the body, we should call oxen happy whenever they come across bitter vetch to eat.'),
        (uuidv7(), 'heraclitus', 'What opposes unites, and the finest attunement stems from things bearing in opposite directions, and all things come about by strife.'),
        (uuidv7(), 'heraclitus', 'For a horse, a dog and a human being have different pleasures; asses prefer straw to gold, since asses find food sweeter than gold.'),
        (uuidv7(), 'epictetus', 'Some things are in our control and others not. Things in our control are opinion, pursuit, desire, aversion, and, in a word, whatever are our own actions. Things not in our control are body, property, reputation, command, and, in one word, whatever are not our own actions.'),
        (uuidv7(), 'epictetus', 'Remember that following desire promises the attainment of that of which you are desirous; and aversion promises the avoiding that to which you are averse. However, he who fails to obtain the object of his desire is disappointed, and he who incurs the object of his aversion wretched.'),
        (uuidv7(), 'epictetus', 'Men are disturbed, not by things, but by the principles and notions which they form concerning things. Death, for instance, is not terrible, else it would have appeared so to Socrates. But the terror consists in our notion of death that it is terrible.'),
        (uuidv7(), 'epictetus', 'Don''t demand that things happen as you wish, but wish that they happen as they do happen, and you will go on well.')
) AS v(id, philosopher_id, quote_text)
WHERE NOT EXISTS (
    SELECT 1 FROM quotes
);