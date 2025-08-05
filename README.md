# music-server

一个基于Rust和actix-web的音乐服务器API项目。

## 功能特点

- 用户认证（注册、登录）
- 歌手管理
- 专辑管理
- 歌曲管理
- JWT身份验证
- 数据库审计日志（创建人、更新人）

## 技术栈

- Rust 语言
- actix-web 框架
- sea-orm 数据库ORM
- SQLite 数据库
- JWT 身份验证

## 项目结构

```
music-server/
├── .env                 # 环境变量配置
├── Cargo.toml           # 项目依赖
├── src/
│   ├── main.rs          # 入口文件
│   ├── config/          # 配置模块
│   ├── handlers/        # 请求处理器
│   ├── middlewares/     # 中间件
│   ├── models/          # 数据模型
│   ├── routers/         # 路由配置
│   └── utils.rs         # 工具函数
└── migration/           # 数据库迁移
```

## 安装指南

1. 克隆仓库
```bash
git clone <repository-url>
cd music-server
```

2. 安装依赖
```bash
cargo build
```

3. 配置环境变量
创建`.env`文件，添加以下内容：
```
DATABASE_URL=sqlite://./music.db
JWT_SECRET=your-secret-key
```

4. 运行数据库迁移
```bash
cd migration
cargo run --bin migration
```

5. 启动服务器
```bash
cargo run
```

## API文档

API接口测试文件位于`bruno/`目录下，可使用Bruno客户端导入测试。

主要API端点：
- POST /api/auth/register - 用户注册
- POST /api/auth/login - 用户登录
- GET /api/artists - 获取歌手列表
- POST /api/artists - 创建歌手
- GET /api/albums - 获取专辑列表
- POST /api/albums - 创建专辑
- GET /api/songs - 获取歌曲列表
- POST /api/songs - 创建歌曲

## 贡献指南

1. Fork 仓库
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建Pull Request

## 许可证

[MIT](LICENSE)