use crate::audio::Audio;
use crate::error::VolumeError;



#[test]
fn test_audio() -> Result<(), VolumeError> {
    let mut audio = Audio::new();
    println!("{}", audio);

    println!("Adjust to '80' in 3 seconds...");
    sleep_3();
    audio.set_volume(80)?;
    println!("{}", audio);
    
    println!("Volume 'mute' in 3 seconds...");
    sleep_3();
    audio.mute()?;
    println!("{}", audio);

    println!("Volume 'unmute' in 3 seconds...");
    sleep_3();
    audio.unmute()?;
    println!("{}", audio);

    println!("Adjust to '50' in 3 seconds...");
    sleep_3();
    audio.set_volume(50)?;
    println!("{}", audio);

    Ok(())
}

fn sleep_3() {
    std::thread::sleep(
        std::time::Duration::new(3,0)
    );
}
