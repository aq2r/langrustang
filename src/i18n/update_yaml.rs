use crate::{i18n::LangYaml, YAML_DATA, YAML_MODIFIED_TIME, YAML_PATH};

/// 更新日時をチェックして更新されていたら static 変数を更新する
pub fn if_update_reload_yaml() -> anyhow::Result<()> {
    let path = {
        let lock = YAML_PATH.lock().unwrap();
        lock.clone()
    };
    let metadata = std::fs::metadata(&path)?;
    let modified_time = metadata.modified()?;

    let cache_modified_time = {
        let lock = YAML_MODIFIED_TIME.read().unwrap();
        lock.clone()
    };

    if modified_time != cache_modified_time {
        // yaml を読み込み
        let yaml_string = std::fs::read_to_string(&path)?;

        // yaml への変換と static への保存
        {
            let mut lock = YAML_DATA.lock().unwrap();

            if let Some(_) = *lock {
                let yaml: LangYaml = serde_yaml::from_str(&yaml_string)?;
                *lock = Some(yaml);
            }
        };

        {
            let mut lock = YAML_MODIFIED_TIME.write().unwrap();
            *lock = modified_time;
        }
    }

    Ok(())
}
