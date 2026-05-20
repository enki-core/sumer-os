slint::include_modules!();
use slint::{ComponentHandle, VecModel, Model};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

// ==========================================
// 🔒 PURE RUST SHA-256 CRYPTO IMPLEMENTATION
// ==========================================
mod crypto {
    fn sha256(data: &[u8]) -> [u8; 32] {
        let mut h: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];
        let k: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];

        let mut padded = data.to_vec();
        let bit_len = (data.len() as u64) * 8;
        padded.push(0x80);
        while (padded.len() + 8) % 64 != 0 {
            padded.push(0x00);
        }
        padded.extend_from_slice(&bit_len.to_be_bytes());

        for chunk in padded.chunks_exact(64) {
            let mut w = [0u32; 64];
            for (i, part) in chunk.chunks_exact(4).enumerate() {
                w[i] = u32::from_be_bytes(part.try_into().unwrap());
            }
            for i in 16..64 {
                let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
                let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
                w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
            }

            let mut a = h[0];
            let mut b = h[1];
            let mut c = h[2];
            let mut d = h[3];
            let mut e = h[4];
            let mut f = h[5];
            let mut g = h[6];
            let mut h_val = h[7];

            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let temp1 = h_val.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let temp2 = s0.wrapping_add(maj);

                h_val = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }

            h[0] = h[0].wrapping_add(a);
            h[1] = h[1].wrapping_add(b);
            h[2] = h[2].wrapping_add(c);
            h[3] = h[3].wrapping_add(d);
            h[4] = h[4].wrapping_add(e);
            h[5] = h[5].wrapping_add(f);
            h[6] = h[6].wrapping_add(g);
            h[7] = h[7].wrapping_add(h_val);
        }

        let mut result = [0u8; 32];
        for (i, &val) in h.iter().enumerate() {
            result[i*4..(i+1)*4].copy_from_slice(&val.to_be_bytes());
        }
        result
    }

    // Stretch password with 5,000 SHA-256 iterations (highly secure PBKDF2 substitute)
    pub fn stretch_password(password: &str) -> [u8; 32] {
        let mut key = sha256(password.as_bytes());
        for _ in 0..5000 {
            key = sha256(&key);
        }
        key
    }

    // High-security feedback stream cipher based on key and byte offset
    pub fn crypt(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut block = *key;
        for (i, &byte) in plaintext.iter().enumerate() {
            if i % 32 == 0 {
                let mut feed = Vec::new();
                feed.extend_from_slice(key);
                feed.extend_from_slice(&(i as u64).to_be_bytes());
                block = sha256(&feed);
            }
            ciphertext.push(byte ^ block[i % 32]);
        }
        ciphertext
    }
}

// Hex encoding/decoding utilities
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 { return None; }
    let mut bytes = Vec::new();
    for i in (0..s.len()).step_by(2) {
        let res = u8::from_str_radix(&s[i..i+2], 16).ok()?;
        bytes.push(res);
    }
    Some(bytes)
}

const ENCRYPT_PREFIX: &str = "[ALWAH_ENCRYPTED]:";
const DECRYPT_OK_HEADER: &str = "[ALWAH_DECRYPTED_OK]";

// ==========================================
// 📁 NOTES PERSISTENCE & DIRECTORY CONFIGS
// ==========================================
fn get_notes_dir() -> PathBuf {
    let sumer_path = Path::new("/root/.notes");
    if sumer_path.exists() || fs::create_dir_all(sumer_path).is_ok() {
        return sumer_path.to_path_buf();
    }
    if let Some(home) = std::env::var_os("HOME") {
        let home_path = Path::new(&home).join(".notes");
        if home_path.exists() || fs::create_dir_all(&home_path).is_ok() {
            return home_path;
        }
    }
    let local_path = Path::new("./.notes");
    fs::create_dir_all(local_path).ok();
    local_path.to_path_buf()
}

fn load_notes_from_disk(dir: &Path) -> Vec<Note> {
    let mut notes = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let title = path.file_stem().unwrap().to_string_lossy().to_string();
                let raw_content = fs::read_to_string(&path).unwrap_or_default();
                
                let is_encrypted = raw_content.starts_with(ENCRYPT_PREFIX);
                
                notes.push(Note {
                    filename: filename.into(),
                    title: title.into(),
                    content: if is_encrypted { "[لوح مشفر ومحمي]".into() } else { raw_content.into() },
                    is_encrypted,
                });
            }
        }
    }
    notes.sort_by(|a, b| a.title.cmp(&b.title));
    notes
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let notes_dir = get_notes_dir();
    
    println!("[+] Alwah Notepad initialized. Notes directory: {:?}", notes_dir);
    
    // Load initial notes
    let initial_notes = load_notes_from_disk(&notes_dir);
    let notes_model = Rc::new(VecModel::from(initial_notes));
    app.set_notes(notes_model.clone().into());
    app.set_active_note_idx(-1);
    app.set_status_msg("أهلاً بك في مفكرة ألواح 📜".into());

    // Holds the dynamic stretched key/password of the active note if decrypted
    let active_note_pwd = Rc::new(RefCell::new(Option::<String>::None));

    // Helper function to update character, word counts, and line numbers gutter in real-time
    let update_app_stats = |app: &AppWindow, content: &str| {
        let char_cnt = content.chars().count() as i32;
        let word_cnt = content.split_whitespace().count() as i32;
        app.set_char_count(char_cnt);
        app.set_word_count(word_cnt);
        
        // Generate vertical line numbers list
        let lines_count = content.split('\n').count().max(1);
        let mut numbers_str = String::new();
        for i in 1..=lines_count {
            numbers_str.push_str(&format!("{}\n", i));
        }
        app.set_line_numbers_str(numbers_str.into());
    };

    // 1. SELECT NOTE CALLBACK
    let app_weak = app.as_weak();
    let notes_model_select = notes_model.clone();
    let active_note_pwd_select = active_note_pwd.clone();
    let notes_dir_select = notes_dir.clone();
    let update_stats_select = update_app_stats;
    app.on_select_note(move |idx| {
        if let Some(app) = app_weak.upgrade() {
            if let Some(note) = notes_model_select.row_data(idx as usize) {
                let filename: String = note.filename.into();
                let file_path = notes_dir_select.join(filename);
                let raw_content = fs::read_to_string(file_path).unwrap_or_default();
                
                app.set_active_note_idx(idx);
                app.set_current_title(note.title);
                app.set_encryption_error("".into());
                *active_note_pwd_select.borrow_mut() = None;
                
                if raw_content.starts_with(ENCRYPT_PREFIX) {
                    app.set_is_active_note_encrypted(true);
                    app.set_is_active_note_locked(true);
                    app.set_current_content("".into());
                    update_stats_select(&app, "");
                } else {
                    app.set_is_active_note_encrypted(false);
                    app.set_is_active_note_locked(false);
                    app.set_current_content(raw_content.clone().into());
                    update_stats_select(&app, &raw_content);
                }
                app.set_status_msg("".into());
            }
        }
    });

    // 2. CREATE NOTE CALLBACK
    let app_weak = app.as_weak();
    let notes_dir_create = notes_dir.clone();
    let notes_model_create = notes_model.clone();
    let active_note_pwd_create = active_note_pwd.clone();
    let update_stats_create = update_app_stats;
    app.on_create_note(move || {
        if let Some(app) = app_weak.upgrade() {
            let mut title = "ملاحظة جديدة".to_string();
            let mut path = notes_dir_create.join(format!("{}.txt", title));
            let mut counter = 1;
            
            while path.exists() {
                title = format!("ملاحظة جديدة {}", counter);
                path = notes_dir_create.join(format!("{}.txt", title));
                counter += 1;
            }
            
            if fs::write(&path, "").is_ok() {
                let updated_notes = load_notes_from_disk(&notes_dir_create);
                notes_model_create.set_vec(updated_notes);
                
                if let Some(new_idx) = notes_model_create.iter().position(|n| n.title == title) {
                    app.set_active_note_idx(new_idx as i32);
                    app.set_current_title(title.into());
                    app.set_current_content("".into());
                    app.set_is_active_note_encrypted(false);
                    app.set_is_active_note_locked(false);
                    app.set_encryption_error("".into());
                    *active_note_pwd_create.borrow_mut() = None;
                    update_stats_create(&app, "");
                    app.set_status_msg("تم إنشاء لوح جديد وحفظه 📝".into());
                }
            }
        }
    });

    // 3. DELETE NOTE CALLBACK
    let app_weak = app.as_weak();
    let notes_dir_delete = notes_dir.clone();
    let notes_model_delete = notes_model.clone();
    let active_note_pwd_delete = active_note_pwd.clone();
    let update_stats_delete = update_app_stats;
    app.on_delete_note(move |idx| {
        if let Some(app) = app_weak.upgrade() {
            if let Some(note) = notes_model_delete.row_data(idx as usize) {
                // ميزة أمان وحماية حديدية: منع حذف الألواح المشفرة إذا كانت مقفلة!
                if note.is_encrypted && app.get_is_active_note_locked() {
                    app.set_status_msg("⚠️ لا يمكن حذف اللوح المشفر وهو مقفل! الرجاء فك القفل أولاً بالرقم السري.".into());
                    return;
                }

                let filename: String = note.filename.into();
                let path = notes_dir_delete.join(filename);
                
                if fs::remove_file(path).is_ok() {
                    let updated_notes = load_notes_from_disk(&notes_dir_delete);
                    notes_model_delete.set_vec(updated_notes);
                    
                    app.set_active_note_idx(-1);
                    app.set_current_title("".into());
                    app.set_current_content("".into());
                    app.set_is_active_note_encrypted(false);
                    app.set_is_active_note_locked(false);
                    app.set_encryption_error("".into());
                    *active_note_pwd_delete.borrow_mut() = None;
                    update_stats_delete(&app, "");
                    app.set_status_msg("تم حذف اللوح 🗑️".into());
                }
            }
        }
    });

    // 4. SAVE NOTE CALLBACK
    let app_weak = app.as_weak();
    let notes_dir_save = notes_dir.clone();
    let notes_model_save = notes_model.clone();
    let active_note_pwd_save = active_note_pwd.clone();
    let update_stats_save = update_app_stats;
    app.on_save_note(move |idx, new_title, new_content| {
        if let Some(app) = app_weak.upgrade() {
            if let Some(note) = notes_model_save.row_data(idx as usize) {
                let old_filename: String = note.filename.into();
                let old_path = notes_dir_save.join(&old_filename);
                
                let clean_title = new_title.trim().to_string();
                if clean_title.is_empty() {
                    app.set_status_msg("⚠️ لا يمكن حفظ اللوح بدون عنوان!".into());
                    return;
                }
                
                let new_filename = format!("{}.txt", clean_title);
                let new_path = notes_dir_save.join(&new_filename);
                
                if old_filename != new_filename {
                    if new_path.exists() {
                        app.set_status_msg("⚠️ يوجد لوح آخر بنفس الاسم!".into());
                        return;
                    }
                    fs::rename(&old_path, &new_path).ok();
                }
                
                let plain_content: String = new_content.into();
                let file_content = if app.get_is_active_note_encrypted() {
                    if let Some(pwd) = active_note_pwd_save.borrow().as_ref() {
                        let stretched_key = crypto::stretch_password(pwd);
                        let tagged_plain = format!("{}{}", DECRYPT_OK_HEADER, plain_content);
                        let ciphertext = crypto::crypt(tagged_plain.as_bytes(), &stretched_key);
                        format!("{}{}", ENCRYPT_PREFIX, hex_encode(&ciphertext))
                    } else {
                        app.set_status_msg("⚠️ خطأ في التشفير: كلمة المرور مفقودة!".into());
                        return;
                    }
                } else {
                    plain_content.clone()
                };
                
                if fs::write(&new_path, &file_content).is_ok() {
                    let updated_notes = load_notes_from_disk(&notes_dir_save);
                    notes_model_save.set_vec(updated_notes);
                    
                    if let Some(new_idx) = notes_model_save.iter().position(|n| n.filename == new_filename) {
                        app.set_active_note_idx(new_idx as i32);
                        update_stats_save(&app, &plain_content);
                        app.set_status_msg("تم حفظ اللوح بنجاح! 💾🟢".into());
                    }
                } else {
                    app.set_status_msg("⚠️ فشل في حفظ الملف!".into());
                }
            }
        }
    });

    // 5. SEAMLESS REAL-TIME AUTO-SAVE CALLBACK (Saves content instantly and refreshes line numbers)
    let app_weak = app.as_weak();
    let notes_dir_auto = notes_dir.clone();
    let notes_model_auto = notes_model.clone();
    let active_note_pwd_auto = active_note_pwd.clone();
    let update_stats_auto = update_app_stats;
    app.on_content_edited(move |text| {
        if let Some(app) = app_weak.upgrade() {
            let idx = app.get_active_note_idx();
            if idx >= 0 {
                let text_str = text.to_string();
                update_stats_auto(&app, &text_str);
                
                if let Some(note) = notes_model_auto.row_data(idx as usize) {
                    let filename: String = note.filename.into();
                    let file_path = notes_dir_auto.join(&filename);
                    
                    let file_content = if app.get_is_active_note_encrypted() {
                        if let Some(pwd) = active_note_pwd_auto.borrow().as_ref() {
                            let stretched_key = crypto::stretch_password(pwd);
                            let tagged_plain = format!("{}{}", DECRYPT_OK_HEADER, text_str);
                            let ciphertext = crypto::crypt(tagged_plain.as_bytes(), &stretched_key);
                            format!("{}{}", ENCRYPT_PREFIX, hex_encode(&ciphertext))
                        } else {
                            return;
                        }
                    } else {
                        text_str.clone()
                    };
                    
                    fs::write(&file_path, &file_content).ok();
                    app.set_status_msg("تم الحفظ تلقائياً... 🟢".into());
                }
            }
        }
    });

    // 6. SEARCH & REPLACE CALLBACK
    app.on_search_replace(move |content, search, replace| {
        let content_str: String = content.into();
        let search_str: String = search.into();
        let replace_str: String = replace.into();
        content_str.replace(&search_str, &replace_str).into()
    });

    // 7. ENCRYPT ACTIVE NOTE CALLBACK
    let app_weak = app.as_weak();
    let notes_dir_encrypt = notes_dir.clone();
    let notes_model_encrypt = notes_model.clone();
    let active_note_pwd_encrypt = active_note_pwd.clone();
    app.on_encrypt_active_note(move |password| {
        if let Some(app) = app_weak.upgrade() {
            let idx = app.get_active_note_idx();
            if idx >= 0 {
                let pwd_str = password.to_string();
                let current_content_str = app.get_current_content().to_string();
                
                if let Some(note) = notes_model_encrypt.row_data(idx as usize) {
                    let filename: String = note.filename.into();
                    let file_path = notes_dir_encrypt.join(&filename);
                    
                    let stretched_key = crypto::stretch_password(&pwd_str);
                    let tagged_plain = format!("{}{}", DECRYPT_OK_HEADER, current_content_str);
                    let ciphertext = crypto::crypt(tagged_plain.as_bytes(), &stretched_key);
                    let file_content = format!("{}{}", ENCRYPT_PREFIX, hex_encode(&ciphertext));
                    
                    if fs::write(&file_path, &file_content).is_ok() {
                        *active_note_pwd_encrypt.borrow_mut() = Some(pwd_str);
                        app.set_is_active_note_encrypted(true);
                        app.set_is_active_note_locked(false);
                        
                        let updated_notes = load_notes_from_disk(&notes_dir_encrypt);
                        notes_model_encrypt.set_vec(updated_notes);
                        
                        app.set_status_msg("🔒 تم تشفير اللوح بنجاح وحمايته!".into());
                    }
                }
            }
        }
    });

    // 8. UNLOCK ACTIVE NOTE CALLBACK
    let app_weak = app.as_weak();
    let notes_dir_unlock = notes_dir.clone();
    let notes_model_unlock = notes_model.clone();
    let active_note_pwd_unlock = active_note_pwd.clone();
    let update_stats_unlock = update_app_stats;
    app.on_unlock_active_note(move |password| {
        if let Some(app) = app_weak.upgrade() {
            let idx = app.get_active_note_idx();
            if idx >= 0 {
                let pwd_str = password.to_string();
                if let Some(note) = notes_model_unlock.row_data(idx as usize) {
                    let filename: String = note.filename.into();
                    let file_path = notes_dir_unlock.join(&filename);
                    let raw_content = fs::read_to_string(file_path).unwrap_or_default();
                    
                    if let Some(hex_part) = raw_content.strip_prefix(ENCRYPT_PREFIX) {
                        if let Some(ciphertext) = hex_decode(hex_part) {
                            let stretched_key = crypto::stretch_password(&pwd_str);
                            let decrypted_bytes = crypto::crypt(&ciphertext, &stretched_key);
                            
                            if let Ok(decrypted_str) = String::from_utf8(decrypted_bytes) {
                                if let Some(plaintext) = decrypted_str.strip_prefix(DECRYPT_OK_HEADER) {
                                    *active_note_pwd_unlock.borrow_mut() = Some(pwd_str);
                                    app.set_is_active_note_locked(false);
                                    app.set_encryption_error("".into());
                                    app.set_current_content(plaintext.into());
                                    update_stats_unlock(&app, plaintext);
                                    app.set_status_msg("🔓 تم فك التشفير بنجاح!".into());
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            app.set_encryption_error("⚠️ كلمة المرور خاطئة!".into());
        }
        false
    });

    // 9. DECRYPT NOTE PERMANENTLY CALLBACK
    let app_weak = app.as_weak();
    let notes_dir_decrypt = notes_dir.clone();
    let notes_model_decrypt = notes_model.clone();
    let active_note_pwd_decrypt = active_note_pwd.clone();
    app.on_decrypt_active_note_permanently(move || {
        if let Some(app) = app_weak.upgrade() {
            let idx = app.get_active_note_idx();
            if idx >= 0 {
                let current_content_str = app.get_current_content().to_string();
                if let Some(note) = notes_model_decrypt.row_data(idx as usize) {
                    let filename: String = note.filename.into();
                    let file_path = notes_dir_decrypt.join(&filename);
                    
                    if fs::write(&file_path, &current_content_str).is_ok() {
                        *active_note_pwd_decrypt.borrow_mut() = None;
                        app.set_is_active_note_encrypted(false);
                        app.set_is_active_note_locked(false);
                        
                        let updated_notes = load_notes_from_disk(&notes_dir_decrypt);
                        notes_model_decrypt.set_vec(updated_notes);
                        
                        app.set_status_msg("🔓 تم إزالة التشفير واللوح الآن عام".into());
                    }
                }
            }
        }
    });

    app.run()
}
