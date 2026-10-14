-- Roller ve izinler (spatie/laravel-permission + Filament Shield düzeni). Veri-yalnız ve yinelenebilir.

INSERT OR IGNORE INTO roles (name, guard_name, created_at, updated_at) VALUES
    ('admin', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('barista', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('menu_manager', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('support', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('content', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('viewer', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('customer', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'));

WITH actions(action) AS (VALUES ('ViewAny'), ('View'), ('Create'), ('Update'), ('Delete'), ('DeleteAny')),
     resources(resource) AS (VALUES ('Urun'), ('Secenek'), ('Musteri'), ('Sube'), ('Kampanya'), ('Siparis'), ('ContactMessage'), ('ContentPage'), ('AuditLog'), ('User'), ('Role'), ('Permission'))
INSERT OR IGNORE INTO permissions (name, guard_name, created_at, updated_at)
SELECT action || ':' || resource, 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now') FROM actions, resources;

INSERT OR IGNORE INTO permissions (name, guard_name, created_at, updated_at) VALUES
    ('Advance:Siparis', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('Cancel:Siparis', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    ('Reveal:PII', 'web', strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'));

-- barista
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'barista' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:Siparis', 'View:Siparis', 'Update:Siparis', 'ViewAny:Urun',
    'View:Urun', 'ViewAny:Secenek', 'View:Secenek', 'ViewAny:Sube',
    'View:Sube', 'Advance:Siparis'
);

-- menu_manager
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'menu_manager' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:Urun', 'View:Urun', 'Create:Urun', 'Update:Urun',
    'Delete:Urun', 'DeleteAny:Urun', 'ViewAny:Secenek', 'View:Secenek',
    'Create:Secenek', 'Update:Secenek', 'Delete:Secenek', 'DeleteAny:Secenek',
    'ViewAny:Kampanya', 'View:Kampanya', 'Create:Kampanya', 'Update:Kampanya',
    'Delete:Kampanya', 'DeleteAny:Kampanya', 'ViewAny:Siparis', 'View:Siparis',
    'ViewAny:Sube', 'View:Sube'
);

-- support
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'support' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:ContactMessage', 'View:ContactMessage', 'Create:ContactMessage', 'Update:ContactMessage',
    'Delete:ContactMessage', 'DeleteAny:ContactMessage', 'ViewAny:Siparis', 'View:Siparis',
    'ViewAny:Musteri', 'View:Musteri', 'Cancel:Siparis'
);

-- content
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'content' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:ContentPage', 'View:ContentPage', 'Create:ContentPage', 'Update:ContentPage',
    'Delete:ContentPage', 'DeleteAny:ContentPage'
);

-- viewer
INSERT OR IGNORE INTO role_has_permissions (permission_id, role_id)
SELECT p.id, r.id FROM permissions p JOIN roles r ON r.name = 'viewer' AND r.guard_name = 'web'
WHERE p.guard_name = 'web' AND p.name IN (
    'ViewAny:Urun', 'View:Urun', 'ViewAny:Secenek', 'View:Secenek',
    'ViewAny:Musteri', 'View:Musteri', 'ViewAny:Sube', 'View:Sube',
    'ViewAny:Kampanya', 'View:Kampanya', 'ViewAny:Siparis', 'View:Siparis',
    'ViewAny:ContactMessage', 'View:ContactMessage', 'ViewAny:ContentPage', 'View:ContentPage'
);
