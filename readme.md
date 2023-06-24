# 使用方法

## 安装
  1. clone本项目到本地
  2. 进入本项目目录
  3. 使用cargo install --path . 默认安装到~/.cargo/bin/目录下

## 使用
  确保minit可以在任意目录下使用，需要将~/.cargo/bin/目录添加到环境变量中
  用 -c 指定配置文件，用 -r 指定管理定义文件的目录，运行结果为根据设置初始化mongodb数据库，形式如下，
  minit -c <knitter/configs.toml> -r <manage_defines_dir>
  
  也可使用-f指定单个管理定义文件，-f和-r不能同时使用。