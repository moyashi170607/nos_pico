use std::env;

fn get_config_toml() -> toml::Value {
    let filename = "config.toml";

    //実行中のexeのフルパスを取得
    let exe_path = env::current_exe().expect("実行ファイルのパス取得に失敗しました");

    // exeのファイル名を取り除き、親ディレクトリを取得
    let exe_dir = exe_path.parent().expect("ディレクトリの取得に失敗しました");

    // ディレクトリのパスに "config.toml" を結合
    let config_path = exe_dir.join(filename);

    let contents =
        std::fs::read_to_string(config_path).expect("設定ファイルの読み込みに失敗しました");

    toml::from_str(&contents).expect("設定ファイルのパースに失敗しました")
}

pub fn get_port_name() -> String {
    let config = get_config_toml();
    let port_name = config
        .get("port_name")
        .and_then(|f| f.as_str())
        .expect("port_nameキーが見つからないか、正しく設定されていません");

    port_name.to_string()
}

pub fn set_relay() -> Vec<String> {
    let config = get_config_toml();

    let relay_array = config
        .get("relays")
        .and_then(|v| v.as_array())
        .expect("relaysキーが見つからないか、配列ではありません");

    let relays: Vec<String> = relay_array
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    return relays;
}
