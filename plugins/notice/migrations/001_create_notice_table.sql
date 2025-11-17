-- 系统通知公告表
CREATE TABLE IF NOT EXISTS `sys_notice` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `title` varchar(128) NOT NULL COMMENT '公告标题',
  `content` text NOT NULL COMMENT '公告内容（富文本）',
  `notice_type` smallint NOT NULL DEFAULT '1' COMMENT '公告类型（1通知/2公告）',
  `level` smallint NOT NULL DEFAULT '1' COMMENT '重要程度（1普通/2重要/3紧急）',
  `is_top` char(1) NOT NULL DEFAULT 'N' COMMENT '是否置顶（Y/N）',
  `status` smallint NOT NULL DEFAULT '0' COMMENT '发布状态（0草稿/1已发布/2已撤回）',
  `publish_time` datetime DEFAULT NULL COMMENT '发布时间',
  `publisher_id` bigint DEFAULT NULL COMMENT '发布人ID',
  `created_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),
  KEY `idx_status` (`status`),
  KEY `idx_is_top` (`is_top`),
  KEY `idx_publish_time` (`publish_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统通知公告表';

-- 插入测试数据
INSERT INTO `sys_notice` (`title`, `content`, `notice_type`, `level`, `is_top`, `status`, `publish_time`, `publisher_id`) VALUES
('系统维护通知', '<p>系统将于本周六晚上22:00-24:00进行例行维护，届时系统将暂停服务，请提前做好相关安排。</p><p>维护期间可能出现的情况：</p><ul><li>系统无法访问</li><li>数据暂时无法更新</li></ul><p>给您带来不便，敬请谅解！</p>', 1, 2, 'Y', 1, NOW(), 1),
('新功能上线公告', '<p>我们很高兴地宣布，系统新增了以下功能：</p><ol><li>代码生成器：快速生成CRUD代码</li><li>数据字典：统一管理系统配置</li><li>系统监控：实时查看系统运行状态</li></ol><p>欢迎大家体验使用！</p>', 2, 1, 'N', 1, NOW(), 1),
('安全更新通知', '<p>系统已完成以下安全更新：</p><ul><li>修复XSS漏洞</li><li>更新依赖包版本</li><li>增强密码加密</li></ul><p>建议所有用户及时更新密码！</p>', 1, 3, 'N', 1, NOW(), 1),
('数据备份提醒', '<p>系统将于每周日凌晨2:00自动进行数据备份，备份期间系统性能可能会有所下降。</p>', 1, 1, 'N', 1, NOW(), 1),
('即将发布的功能预告', '<p>下一版本计划发布的新功能：</p><ol><li>支持多语言</li><li>移动端适配</li><li>数据导入导出</li></ol><p>敬请期待！</p>', 2, 1, 'N', 0, NULL, NULL);
