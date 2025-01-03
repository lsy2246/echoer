use super::error::{CustomErrorInto, CustomResult};
use web_sys::{window, Document, Element, Storage, Window};

fn get_window() -> CustomResult<Window> {
    Ok(window().ok_or("浏览器window对象不存在")?)
}

fn get_storage() -> CustomResult<Storage> {
    get_window()?
        .local_storage()?
        .ok_or("获取浏览器Storge对象失败".into_custom_error())
}
fn get_document() -> CustomResult<Document> {
    get_window()?
        .document()
        .ok_or("获取浏览器Document对象失败".into_custom_error())
}

fn get_element(ele_name: &str) -> CustomResult<Element> {
    get_document()?
        .query_selector(ele_name)?
        .ok_or(format!("获取元素{}失败", ele_name).into_custom_error())
}

pub fn get_local_storage_value(key: &str) -> CustomResult<String> {
    get_storage()?
        .get_item(key)?
        .ok_or(format!("localStorage中不存在键'{}'", key).into_custom_error())
}

pub fn set_local_storage_value(key: &str, value: &str) -> CustomResult<()> {
    Ok(get_storage()?.set_item(key, value)?)
}

pub fn remove_local_storage_value(key: &str) -> CustomResult<()> {
    Ok(get_storage()?.remove_item(key)?)
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

pub fn add_element_class(ele_name: &str, class_name: &str) -> CustomResult<()> {
    Ok(get_element(ele_name)?.class_list().add_1(class_name)?)
}

pub fn remove_element_class(ele_name: &str, class_name: &str) -> CustomResult<()> {
    Ok(get_element(ele_name)?.class_list().remove_1(class_name)?)
}

pub fn remove_element(ele_name: &str) -> CustomResult<()> {
    let e = get_element(ele_name)?;
    let _ = e.parent_node().ok_or("无法获取父节点")?.remove_child(&e)?;
    Ok(())
}
