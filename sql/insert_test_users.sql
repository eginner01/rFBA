-- 插入测试用户数据
-- 密码格式: salt:hash (bcrypt)
-- 密码 admin123 的哈希值示例（需要使用真实的bcrypt计算）

-- 管理员用户
INSERT INTO `sys_user` (
    `id`, `uuid`, `username`, `nickname`, `password`, `salt`,
    `email`, `phone`, `avatar`, `status`, `is_superuser`, `is_staff`, `is_multi_login`,
    `join_time`, `created_time`, `updated_time`, `del_flag`
) VALUES (
    1,
    REPLACE(UUID(), '-', ''),
    'admin',
    '系统管理员',
    '$2b$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', -- password (bcrypt hash of "admin123")
    'admin_salt_2024',
    'admin@example.com',
    '13800138000',
    NULL,
    1,
    1,
    1,
    1,
    NOW(),
    NOW(),
    NOW(),
    0
) ON DUPLICATE KEY UPDATE
    `password` = VALUES(`password`),
    `salt` = VALUES(`salt`),
    `updated_time` = NOW();

-- 普通用户
INSERT INTO `sys_user` (
    `id`, `uuid`, `username`, `nickname`, `password`, `salt`,
    `email`, `phone`, `avatar`, `status`, `is_superuser`, `is_staff`, `is_multi_login`,
    `join_time`, `created_time`, `updated_time`, `del_flag`
) VALUES (
    2,
    REPLACE(UUID(), '-', ''),
    'user',
    '普通用户',
    '$2b$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', -- password (bcrypt hash of "admin123")
    'user_salt_2024',
    'user@example.com',
    '13900139000',
    NULL,
    1,
    0,
    1,
    1,
    NOW(),
    NOW(),
    NOW(),
    0
) ON DUPLICATE KEY UPDATE
    `password` = VALUES(`password`),
    `salt` = VALUES(`salt`),
    `updated_time` = NOW();
