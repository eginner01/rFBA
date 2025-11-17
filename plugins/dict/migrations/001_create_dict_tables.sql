-- 字典类型表
CREATE TABLE IF NOT EXISTS `sys_dict_type` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `name` varchar(32) NOT NULL COMMENT '字典名称',
  `code` varchar(32) NOT NULL COMMENT '字典编码',
  `status` smallint NOT NULL DEFAULT '1' COMMENT '状态（1启用/0禁用）',
  `remark` text COMMENT '备注',
  `created_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_code` (`code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统字典类型表';

-- 字典数据表
CREATE TABLE IF NOT EXISTS `sys_dict_data` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `label` varchar(64) NOT NULL COMMENT '显示标签',
  `value` varchar(64) NOT NULL COMMENT '数据值',
  `sort` int NOT NULL DEFAULT '0' COMMENT '排序号',
  `type_id` bigint NOT NULL COMMENT '所属字典类型ID',
  `type_code` varchar(32) NOT NULL COMMENT '所属字典类型编码',
  `is_default` char(1) NOT NULL DEFAULT 'N' COMMENT '是否默认（Y/N）',
  `status` smallint NOT NULL DEFAULT '1' COMMENT '状态（1启用/0禁用）',
  `remark` text COMMENT '备注',
  `created_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_type_id` (`type_id`),
  KEY `idx_type_code` (`type_code`),
  CONSTRAINT `fk_dict_data_type` FOREIGN KEY (`type_id`) REFERENCES `sys_dict_type` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统字典数据表';

-- 插入测试数据
INSERT INTO `sys_dict_type` (`name`, `code`, `status`, `remark`) VALUES
('用户状态', 'user_status', 1, '用户账户状态'),
('角色类型', 'role_type', 1, '系统角色类型'),
('菜单类型', 'menu_type', 1, '系统菜单类型');

INSERT INTO `sys_dict_data` (`label`, `value`, `sort`, `type_id`, `type_code`, `is_default`, `status`) VALUES
('正常', '1', 1, 1, 'user_status', 'Y', 1),
('禁用', '0', 2, 1, 'user_status', 'N', 1),
('管理员', '1', 1, 2, 'role_type', 'Y', 1),
('普通用户', '2', 2, 2, 'role_type', 'N', 1),
('目录', 'C', 1, 3, 'menu_type', 'N', 1),
('菜单', 'M', 2, 3, 'menu_type', 'N', 1),
('按钮', 'F', 3, 3, 'menu_type', 'N', 1);
