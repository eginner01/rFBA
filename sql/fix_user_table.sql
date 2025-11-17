-- 修复用户表缺失列
-- 执行时间: 2025-11-14

USE fba;

-- 检查sys_user表是否存在del_flag列，如果不存在则添加
SET @sql = CONCAT('ALTER TABLE sys_user
ADD COLUMN IF NOT EXISTS `del_flag` tinyint(1) NOT NULL DEFAULT ''0'' COMMENT ''删除标志(0代表未删除,2代表已删除)'' AFTER `updated_time`,
ADD COLUMN IF NOT EXISTS `salt` varchar(100) DEFAULT NULL COMMENT ''密码盐'' AFTER `password`;');

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- 插入默认管理员用户（如果不存在）
INSERT IGNORE INTO `sys_user` (
    `id`, `uuid`, `username`, `nickname`, `password`, `salt`,
    `email`, `phone`, `avatar`, `status`, `is_superuser`, `is_staff`, `is_multi_login`,
    `join_time`, `created_time`, `updated_time`, `del_flag`
) VALUES (
    1,
    REPLACE(UUID(), '-', ''),
    'admin',
    '系统管理员',
    '$2b$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi',
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
);

-- 确认插入结果
SELECT id, username, nickname, status, is_superuser, del_flag FROM sys_user WHERE username = 'admin';
