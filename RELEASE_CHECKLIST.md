# Homebrew 发布检查清单

## 发布前检查

- [ ] 代码已准备好发布
- [ ] 所有测试都通过
- [ ] 文档已更新
- [ ] 版本号已确定
- [ ] 已安装 `gh` CLI 并认证
- [ ] GitHub 仓库设置正确

## 发布步骤

- [ ] 运行发布脚本：`./scripts/release.sh <version>`
- [ ] 确认 GitHub Actions 构建成功
- [ ] 检查 GitHub Release 页面
- [ ] 验证所有平台的二进制文件都已上传

## 创建 Homebrew Tap

- [ ] 创建 `homebrew-mxlogger` 仓库
- [ ] 复制 formula 到新仓库
- [ ] 推送 formula 到 tap 仓库

## 测试安装

- [ ] 在干净的环境中测试：`brew tap suyulin/mxlogger`
- [ ] 安装工具：`brew install mxlogger-analyzer-rust`
- [ ] 验证工具功能：`mxlogger_analyzer_rust --help`
- [ ] 测试基本功能

## 发布后

- [ ] 更新项目文档
- [ ] 通知用户（如果适用）
- [ ] 监控 GitHub issues 中的反馈
- [ ] 准备下一个版本的开发

## 故障排除检查

- [ ] 如果 GitHub Actions 失败，检查日志
- [ ] 如果 SHA256 不匹配，重新运行更新脚本
- [ ] 如果 Homebrew 安装失败，检查 formula 语法
- [ ] 如果权限问题，检查二进制文件权限

## 文件验证

确保以下文件存在且正确：
- [ ] `.github/workflows/release.yml`
- [ ] `Formula/mxlogger-analyzer-rust.rb`
- [ ] `scripts/release.sh`
- [ ] `scripts/update_formula.sh`
- [ ] `README.md` (已更新安装说明)
- [ ] `README_zh.md` (已更新安装说明)
