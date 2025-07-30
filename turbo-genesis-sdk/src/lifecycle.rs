pub fn on_before_hot_reload() -> Result<(), std::io::Error> {
    crate::camera::on_before_hot_reload()?;
    Ok(())
}

pub fn on_after_hot_reload() -> Result<(), std::io::Error> {
    crate::camera::on_after_hot_reload()?;
    Ok(())
}

pub fn on_reset() -> Result<(), std::io::Error> {
    crate::camera::on_reset()?;
    Ok(())
}

pub fn on_update() -> Result<(), std::io::Error> {
    crate::camera::on_update()?;
    Ok(())
}
