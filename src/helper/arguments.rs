use clap::{ArgMatches, clap_app};

thread_local! {
  pub static MATCHES: ArgMatches = clap_app!(db_sync_tool =>
    (version: "1.0")
    (author: "Herb Luo <cloudself.cn@gmail.com>")
    (about: "A tool for sync db")
    (@arg ADDRESS: -a --address +takes_value default_value["0.0.0.0"] "自定义服务绑定的地址，默认0.0.0.0")
    (@arg PORT: -p --port +takes_value default_value["9886"] "自定义端口，默认9886")
    (@arg SHOW_CONFIG: -s --show-config +takes_value "显示目前的配置")
  ).get_matches();
}

pub fn init() {
  info!("matches: {:?}", MATCHES);
}
