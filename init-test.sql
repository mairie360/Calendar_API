-- Test fixtures run by the `seeder` service once Liquibase is done (dev, integration,
-- performance and security stacks).
--
-- User 1 (Admin) is already created by the `create_admin` changeset of the liquibase-migrations
-- image; it is the `sub` of the JWT ZAP injects. User 2 is a plain `User` account (event
-- participant). User 3 is a `Responsable` sharing a group with user 2: an event created by user 2
-- with user 3 as participant goes through the validation circuit (PATCH /events/{id}/validation,
-- exercised by load-test.js).
--
-- User 51 and event 21 are the ids of the path parameter examples of the spec: ZAP builds its
-- requests from these examples, so seeding them makes it scan the handlers on a real event
-- (the Admin and user 51 assigned) instead of stopping at a 404.
--
-- The password is the public argon2id hash of the template admin account: `users.password` only
-- accepts argon2id PHC strings since database 1.3.0 (chk_users_password_hashed).

INSERT INTO users (id, first_name, last_name, email, password, status, is_archived)
VALUES
    (2, 'Test', 'User', 'test2@mairie360.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$/iKF9PbiDRDs4EKPjlIIhg$UKx9vfwwps250mEP/bYp63CXbEnQGULeUAhDq+az9Aw',
     'active', FALSE),
    (3, 'Test', 'Responsable', 'test3@mairie360.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$/iKF9PbiDRDs4EKPjlIIhg$UKx9vfwwps250mEP/bYp63CXbEnQGULeUAhDq+az9Aw',
     'active', FALSE),
    (51, 'Amina', 'Bensaïd', 'amina.bensaid@mairie360.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$/iKF9PbiDRDs4EKPjlIIhg$UKx9vfwwps250mEP/bYp63CXbEnQGULeUAhDq+az9Aw',
     'active', FALSE)
ON CONFLICT (id) DO NOTHING;

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM roles r JOIN (VALUES (2, 'User'), (3, 'Responsable'), (51, 'User')) AS u(id, role) ON r.name = u.role
ON CONFLICT DO NOTHING;

-- Group shared by user 2 and the Responsable (user 3), owned by user 2.
INSERT INTO groups (id, owner_id, name, description)
VALUES (1000, 2, 'k6 validation circuit', 'Shared by the event creator and the Responsable validating it')
ON CONFLICT (id) DO NOTHING;

INSERT INTO group_members (group_id, user_id) VALUES (1000, 2), (1000, 3)
ON CONFLICT DO NOTHING;

SELECT setval(pg_get_serial_sequence('groups', 'id'), GREATEST((SELECT MAX(id) FROM groups), 1));

INSERT INTO events (id, name, description, start_date, end_date, visibility, category, location,
                    created_by, owner_id)
VALUES (21, 'Conseil municipal', 'Ordre du jour envoyé une semaine avant', '2026-10-05 18:00:00+00',
        '2026-10-05 20:00:00+00', 'public', 'meeting', 'Salle du conseil', 1, 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO event_members (event_id, user_id, validation_status)
SELECT 21, u.id, 'validated' FROM (VALUES (1), (51)) AS u(id)
WHERE NOT EXISTS (SELECT 1 FROM event_members m WHERE m.event_id = 21 AND m.user_id = u.id);

SELECT setval(pg_get_serial_sequence('events', 'id'), GREATEST((SELECT MAX(id) FROM events), 1));

-- Explicit ids do not advance the SERIAL sequence: move it past the fixtures so users created
-- later do not collide with them.
SELECT setval(pg_get_serial_sequence('users', 'id'), GREATEST((SELECT MAX(id) FROM users), 1));
