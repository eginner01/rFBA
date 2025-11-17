-- 邮件发送记录表
CREATE TABLE IF NOT EXISTS `sys_email_record` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `to_email` varchar(255) NOT NULL COMMENT '收件人邮箱',
  `subject` varchar(255) NOT NULL COMMENT '邮件主题',
  `content` text NOT NULL COMMENT '邮件内容',
  `is_html` smallint NOT NULL DEFAULT '0' COMMENT '是否HTML格式（1是/0否）',
  `status` smallint NOT NULL DEFAULT '0' COMMENT '发送状态（0待发送/1发送成功/2发送失败）',
  `error_msg` text COMMENT '错误信息',
  `send_time` datetime DEFAULT NULL COMMENT '发送时间',
  `created_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `idx_to_email` (`to_email`),
  KEY `idx_status` (`status`),
  KEY `idx_created_time` (`created_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='邮件发送记录表';
