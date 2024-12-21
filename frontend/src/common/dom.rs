use super::error::{CustomErrorInto, CustomResult};
use web_sys::{window, Document, MediaQueryList, Storage, Window};

fn get_window() -> CustomResult<Window> {
    Ok(window().ok_or("浏览器window对象不存在")?)
}

fn get_storage() -> CustomResult<Storage> {
    get_window()?
        .local_storage()?
        .ok_or("浏览器不支持localStorage".into_custom_error())
}

pub fn get_local_storage_value(key: &str) -> CustomResult<String> {
    get_storage()?
        .get_item(key)?
        .ok_or(format!("localStorage中不存在键'{}'", key).into_custom_error())
}

pub fn set_local_storage_value(key: &str, value: &str) -> CustomResult<()> {
    Ok(get_storage()?.set_item(key, value)?)
}

pub fn get_media_theme() -> CustomResult<String> {
    let media_query = get_window()?
        .match_media("(prefers-color-scheme: dark)")?
        .ok_or("查询media时发生错误".into_custom_error())?
        .matches();
    if media_query {
        return Ok("dark".to_string());
    }
    Ok("light".to_string())
}

pub fn set_element_class(ele_name: &str, class_name: &str) -> CustomResult<()> {
    get_window()?
        .document()
        .ok_or("浏览器document对象不存在".into_custom_error())?
        .query_selector(ele_name)?
        .ok_or(format!("获取元素{}失败", ele_name).into_custom_error())?
        .set_class_name(class_name);

    Ok(())
}
