use rdev::{listen, Event};
use std::fs::{File,write};
use rustc_serialize::json::Json;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::BTreeMap;
use cocoa::appkit::{NSApp, NSPasteboard, NSPasteboardTypeString};
use cocoa::foundation::{NSArray, NSString};
use std::{str, slice};


static mut CLIPBOARD : &str = "";

struct Logger{
    json : Json,
    file : String
}

impl Logger{

    fn log(&mut self, msg : &str, title : &str){
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("LOG || {} || {:?} || {}", &title.to_ascii_uppercase(), &msg, since_the_epoch);
        let mut test: BTreeMap<String, Json> = BTreeMap::new();
        test.insert("msg".to_string(), Json::String(msg.to_string()));
        test.insert("title".to_string(), Json::String(title.to_string().to_ascii_uppercase()));
        self.json.as_object_mut().unwrap().insert(since_the_epoch.to_string(), Json::Object(test));
        self.update();
    }

    fn update(&self){
        write(&self.file, self.json.pretty().to_string()).unwrap();

    }

    fn start(&mut self, json : Json, file : String){
        self.json = json;
        self.file = file;
        self.log("Starting Logger", "start")
    }
}

static mut LOGGER : Logger =  Logger{json:Json::Boolean(false),file:String::new()};

fn callback(event: Event){
    let result = match event.name {
        Some(string) => string,
        None => String::new()
    };
    if !result.is_empty(){
        unsafe {
            LOGGER.log(result.as_str(),"button clicked");
            let app = NSApp();
            let pid = NSPasteboard::generalPasteboard(app);
            let nsarray_ptr = pid.pasteboardItems();
            if nsarray_ptr.count() != 0 && result == "c"{
                let raw_item_ptr = NSArray::objectAtIndex(nsarray_ptr, 0);
                let itm = raw_item_ptr.stringForType(NSPasteboardTypeString);

                let stri = itm.UTF8String() as *const u8;
                let newclipboard = str::from_utf8(slice::from_raw_parts(stri, itm.len()))
                .unwrap();
                if newclipboard != CLIPBOARD{
                    CLIPBOARD = newclipboard;
                    LOGGER.log(newclipboard, "clipboard changed")
                }
            }
        }
    }   
}    

fn main() {
    let mut file = File::open("logs.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json = Json::from_str(&data).unwrap();
    unsafe {
        LOGGER.start(json, "logs.json".to_string());
    }
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}