use dependencies_sync::bson::doc;
use dependencies_sync::clap::{Arg, Command};
use dependencies_sync::tokio;

use manage_define::manage_ids::*;

use define_utils as utils;

mod init_basic_items;
mod init_manages_db;
mod init_root_password;
mod init_view_rules;

use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n, t};
i18n!("locales");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_matches = Command::new("Manager Init")
        .arg(
            Arg::new("debug")
                .help("turn on debugging information")
                .short('d'),
        )
        .args(&[
            // 数据库地址
            Arg::new("configs")
                .help("configs file path")
                .takes_value(true)
                .short('c')
                .long("configs"),
            // 指定单个文件
            Arg::new("file")
                .help("manage toml file")
                .takes_value(true)
                .short('f')
                .long("file"),
            // 指定目录
            Arg::new("directory")
                .help("toml files directory")
                .takes_value(true)
                .short('r')
                .long("directory"),
            // 指定根用户密码
            Arg::new("rpasswd")
                .help("specify root password, default \'root\'")
                .takes_value(true)
                .short('p')
                .long("rpasswd"),
        ])
        .get_matches();

    // 没有指定管理定义则退出
    if !arg_matches.is_present("file")
        && !arg_matches.is_present("directory")
        && !arg_matches.is_present("configs")
    {
        panic!(
            "{}",
            t!("需要指定项目配置文件、定义文件或者包含定义文件的目录")
        );
    }

    if let Some(cfg_path) = arg_matches.value_of("configs") {
        configs::init_configs_path(cfg_path.to_string())
            .expect(t!("初始化设置文件路径失败").as_str());
    }

    // 数据库检查
    let db = database::get_cashmere_database().await;
    let db_name = db.name();
    println!("{}：{}", t!("连接到数据库"), db_name);

    // 文件列表
    let mut toml_pathes: Vec<String> = vec![];
    println!("------{}------\n", t!("开始读取定义文件"));
    // 添加单文件
    if let Some(path) = arg_matches.value_of("file") {
        toml_pathes.push(path.to_string());
    }
    // 添加目录
    if let Some(path) = arg_matches.value_of("directory") {
        let mut tomls = utils::get_toml_files_of_dir(&path.to_string()).unwrap();
        toml_pathes.append(&mut tomls);
    }

    // 读入所有文件并构造toml映射
    let tomls = utils::get_tomls_from_pathes(&toml_pathes).unwrap();
    println!("------{}-----\n\n", t!("读取定义文件完成"));

    // 使用root用户和root组初始化管理数据库
    let root_id = &"8610000000000".to_string();
    let root_group_id = &"10000".to_string();
    let root_password = arg_matches.value_of("rpasswd").map(|p| p.to_string());

    // 1. 创建管理集合
    match db
        .create_collection(&MANAGES_MANAGE_ID.to_string(), None)
        .await
    {
        Err(e) => println!(
            "{}: {} {:?}",
            t!("创建管理集合失败，集合可能已存在"),
            MANAGES_MANAGE_ID,
            e
        ),
        _ => println!("创建管理集合成功: {}", MANAGES_MANAGE_ID),
    }

    println!("------{}-------", t!("开始初始化管理数据库"));
    init_manages_db::init_manages_db(db, &tomls, root_id, root_group_id).await;
    println!("------{}-------\n", t!("初始化管理数据库完成"));

    // 2. 添加初始实体数据
    println!("------{}-------", t!("开始插入初始数据"));
    init_basic_items::init_basic_items(&tomls, root_id, root_group_id).await;
    println!("------{}-------\n", t!("插入初始数据结束"));

    // 3. 添加映像规则
    println!("------{}-------", t!("开始添加映像规则"));
    init_view_rules::init_view_rules(&tomls, root_id, root_group_id).await;
    println!("------{}-------\n", t!("添加映像规则完成"));

    // 初始化根用户密码
    println!("------{}-------", t!("开始初始化根用户"));
    init_root_password::init_root_password(&root_id, &root_password).await;
    println!("------{}-------\n", t!("初始化根用户完成"));

    // tokio::join!(init_manages_db, init_event_types, init_basic_items, init_view_rules, init_root_password);

    Ok(())
}
