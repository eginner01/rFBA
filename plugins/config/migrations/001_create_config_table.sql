-- 系统配置表
CREATE TABLE IF NOT EXISTS `sys_config` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `name` varchar(64) NOT NULL COMMENT '配置名称',
  `key` varchar(64) NOT NULL COMMENT '配置键',
  `value` text NOT NULL COMMENT '配置值',
  `config_type` varchar(32) NOT NULL DEFAULT 'text' COMMENT '配置类型（text/number/boolean/json/array）',
  `is_frontend` smallint NOT NULL DEFAULT '0' COMMENT '是否前端可见（1可见/0不可见）',
  `status` smallint NOT NULL DEFAULT '1' COMMENT '状态（1启用/0禁用）',
  `remark` text COMMENT '备注',
  `created_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_key` (`key`),
  KEY `idx_is_frontend` (`is_frontend`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统配置表';

-- 插入测试数据
INSERT INTO `sys_config` (`name`, `key`, `value`, `config_type`, `is_frontend`, `status`, `remark`) VALUES
('系统名称', 'system.name', 'FastAPI最佳架构', 'text', 1, 1, '系统显示名称'),
('系统版本', 'system.version', '1.0.0', 'text', 1, 1, '系统版本号'),
('系统Logo', 'system.logo', '/logo.png', 'text', 1, 1, '系统Logo路径'),
('用户注册', 'user.register', 'true', 'boolean', 1, 1, '是否允许用户注册'),
('登录验证码', 'login.captcha', 'true', 'boolean', 1, 1, '登录时是否需要验证码'),
('会话超时时间', 'session.timeout', '3600', 'number', 0, 1, '会话超时时间（秒）'),
('上传文件大小限制', 'upload.max_size', '10485760', 'number', 0, 1, '上传文件最大大小（字节）'),
('允许的文件类型', 'upload.allowed_types', '["jpg","png","pdf","doc"]', 'json', 0, 1, '允许上传的文件类型');
