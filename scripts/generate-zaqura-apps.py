#!/usr/bin/env python3
"""Regenerate programs/zaqura-store/ui/data/apps_data.slint from programs/ folders."""
import hashlib
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PROGRAMS = os.path.join(ROOT, "programs")
OUT = os.path.join(PROGRAMS, "zaqura-store", "ui", "data", "apps_data.slint")

EXTERNAL = [
    ("telegram-desktop", "تليجرام ديسكتاب (Telegram)", "external", "💬",
     "تطبيق المحادثات والرسائل الفوري والمحمي الشهير عالمياً.",
     "تليجرام ديسكتاب للمراسلة الفورية والملفات السحابية.",
     "Telegram FZ-LLC", "v4.8.1", "update_available", "#0ea5e9"),
    ("chromium", "متصفح كروميوم (Chromium)", "external", "🌐",
     "نسخة مفتوحة المصدر من متصفح كروم لتصفح آمن وسريع.",
     "متصفح كروميوم مع دعم الإضافات وسجل التصفح.",
     "Google / Chromium Project", "v112.0", "available", "#2563eb"),
    ("code-oss", "فيجوال ستوديو كود (VS Code)", "external", "💻",
     "محرر أكواد للمطورين بجميع اللغات.",
     "بيئة برمجة متكاملة على سومر OS.",
     "Microsoft", "v1.78.0", "available", "#007acc"),
    ("discord", "ديسكورد (Discord)", "external", "🎮",
     "منصة التواصل الصوتي والكتابي.",
     "قنوات صوتية ودردشة ومشاركة شاشة.",
     "Discord Inc.", "v1.0.9", "available", "#5865f2"),
    ("vlc", "مشغل VLC (VLC)", "external", "🎬",
     "مشغل وسائط وفيديو مفتوح المصدر.",
     "يشغّل جميع صيغ الفيديو والترجمات.",
     "VideoLAN", "v3.0.18", "available", "#ea580c"),
    ("gimp", "GIMP للرسوم", "external", "🎨",
     "تعديل وتصميم صور احترافي.",
     "بديل مجاني للفوتوشوب.",
     "GIMP Project", "v2.10.34", "available", "#a855f7"),
]

KNOWN = {
    "ur-desktop": ("ديسكتاب أور (Ur Desktop)", "واجهة سطح المكتب الرئيسية لسومر OS.",
        "لوحة تحكم، شريط مهام، ومشغلات متكاملة.", "v1.5.0", "installed", "#6366f1", "🏛️"),
    "enki-calc": ("حاسبة إنكي (Enki Calc)", "آلة حاسبة علمية متطورة.",
        "دوال متقدمة ومبنية بـ Rust.", "v1.0.0", "installed", "#38bdf8", "🧮"),
    "alwah-notepad": ("مفكرة ألواح (Alwah Notepad)", "محرر نصوص مع تشفير SHA-256.",
        "تشفير وأرقام أسطر وتصميم متجاوب.", "v1.2.0", "installed", "#3b82f6", "📜"),
    "tigris-shell": ("شيل دجلة (Tigris Shell)", "محاكي طرفية تفاعلي.",
        "واجهة أوامر مع اقتراحات ذكية.", "v1.0.5", "installed", "#10b981", "🐚"),
    "orok-browser": ("متصفح أوروك (Orok Browser)", "متصفح ويب سريع وآمن.",
        "تبويبات متعددة وشريط عناوين ذكي.", "v0.9.8", "installed", "#f59e0b", "⛵"),
    "nahr-tv": ("تلفاز نهر (Nahr TV)", "واجهة ترفيهية للشاشات الكبيرة.",
        "أفلام وأغاني وتنقل سلس.", "v1.1.0", "installed", "#8b5cf6", "📺"),
    "afak-mp4": ("مشغل آفاق (Afak MP4)", "مشغل فيديو وصوتيات.",
        "تشغيل MP4 وترجمات بجودة عالية.", "v1.0.0", "available", "#ec4899", "🎬"),
    "arena-gamecenter": ("صالة أرينا (Arena Game Center)", "مركز ألعاب سومر.",
        "تشغيل وتحميل ألعاب تفاعلية.", "v1.2.0", "available", "#ef4444", "🕹️"),
    "atlas-filemanager": ("أطلس لإدارة الملفات", "مستكشف ملفات متطور.",
        "تصفح ونسخ وضغط المجلدات.", "v1.1.0", "installed", "#14b8a6", "📁"),
    "cipher-passwordmanager": ("خزنة سايفر (Cipher)", "مدير كلمات مرور مشفّر.",
        "حفظ بيانات حساسة بأمان.", "v1.0.0", "available", "#84cc16", "🔐"),
    "crypta-file2pass": ("كريبتا (Crypta)", "تشفير الملفات بكلمة مرور.",
        "حماية خصوصية الملفات.", "v1.0.5", "available", "#10b981", "🛡️"),
    "ember-isowriter": ("حارق إمبر (Ember ISO)", "حرق ISO على USB.",
        "إنشاء وسائط إقلاع.", "v1.0.0", "available", "#f97316", "🔥"),
    "flux-editor": ("محرر فلوكس (Flux Editor)", "محرر صور ووسائط.",
        "فلاتر وتعديل ألوان.", "v1.0.0", "available", "#d946ef", "🎨"),
    "helix-set": ("إعدادات هيليكس", "لوحة تحكم النظام.",
        "شبكة، مظهر، وأمان.", "v1.3.0", "installed", "#475569", "⚙️"),
    "lens-searchengine": ("محرك لينس (Lens)", "بحث ملفات فوري.",
        "فهرسة سريعة داخل النظام.", "v1.0.0", "installed", "#0ea5e9", "🔍"),
    "linka-link2phone": ("لينكا (Link2Phone)", "ربط الهاتف بالنظام.",
        "مزامنة إشعارات وملفات.", "v1.1.0", "available", "#3b82f6", "📱"),
    "memo-clipboard": ("حافظة ميمو", "مدير الحافظة الذكية.",
        "حفظ النصوص والروابط المنسوخة.", "v1.0.0", "installed", "#eab308", "📋"),
    "mesh-networkmanager": ("مدير شبكة ميش", "إدارة الواي فاي.",
        "فحص الإشارة والاتصال.", "v1.0.0", "installed", "#22c55e", "📶"),
    "nexus-extensionsmanager": ("نكسس للإضافات", "مدير إضافات وسمات.",
        "ودجات وسمات سطح المكتب.", "v1.0.0", "installed", "#a855f7", "🧩"),
    "prism-screenshot": ("بريزم للقطات الشاشة", "التقاط الشاشة.",
        "لقطة كاملة أو نافذة محددة.", "v1.0.0", "installed", "#6366f1", "📸"),
    "pulse-notification": ("بولس للإشعارات", "مركز إشعارات موحّد.",
        "تنبيهات التطبيقات والنظام.", "v1.0.0", "installed", "#f43f5e", "🔔"),
    "raneem-mp3": ("مشغل رنيم (MP3)", "مشغل موسيقى.",
        "قوائم تشغيل وموازن صوت.", "v1.1.0", "available", "#06b6d4", "🎵"),
    "relic-archivemanager": ("ريليك للأرشيف", "ضغط وفك ضغط الملفات.",
        "zip و tar.gz وغيرها.", "v1.0.0", "available", "#64748b", "📦"),
    "rune-ai": ("مساعد رون (AI)", "مساعد ذكاء اصطناعي.",
        "إجابات وأكواد ونصوص.", "v2.0.0", "installed", "#8b5cf6", "🔮"),
    "slint-viewer": ("مستعرض Slint", "معاينة واجهات Slint.",
        "تطوير واجهات حياً.", "v1.5.0", "installed", "#ec4899", "🖥️"),
    "zaqura-store": ("متجر زقورة", "متجر تطبيقات سومر.",
        "تثبيت وتحديث البرامج.", "v1.0.0", "installed", "#f59e0b", "🏪"),
    "abzu-downloadmanager": ("مدير التحميل أبزو", "إدارة التنزيلات.",
        "تنزيلات منظمة وسريعة.", "v0.1.0", "available", "#0ea5e9", "⬇️"),
    "adad-routermanager": ("مدير الموجه أداد", "إعدادات الراوتر.",
        "شبكة واتصال منزلي.", "v0.1.0", "available", "#3b82f6", "📡"),
    "Anu-Lock": ("قفل أنو (Anu Lock)", "قفل الشاشة والجلسة.",
        "حماية الوصول للنظام.", "v0.1.0", "available", "#ef4444", "🔒"),
    "apsu-cast": ("بث أبسو (Apsu Cast)", "بث الشاشة والوسائط.",
        "عرض لاسلكي للمحتوى.", "v0.1.0", "available", "#8b5cf6", "📡"),
    "ashur-startupprograms": ("برامج بدء أشور", "إدارة بدء التشغيل.",
        "تطبيقات تلقائية عند الإقلاع.", "v0.1.0", "available", "#10b981", "🚀"),
    "dubbin-printmanager": ("مدير الطباعة دبّين", "إدارة الطابعات.",
        "طباعة ومتابعة المهام.", "v0.1.0", "available", "#64748b", "🖨️"),
    "enlil-firewall": ("جدار إنليل", "جدار ناري للنظام.",
        "حماية الشبكة والمنافذ.", "v0.1.0", "available", "#f97316", "🛡️"),
    "erra-fileshare": ("مشاركة إرا", "مشاركة ملفات محلية.",
        "نقل بين الأجهزة.", "v0.1.0", "available", "#14b8a6", "🔗"),
    "Gilgamesh-Clean": ("منظّف جيلغامش", "تنظيف النظام.",
        "مسح مؤقت وملفات زائدة.", "v0.1.0", "available", "#84cc16", "🧹"),
    "girsu-qrcode": ("رمز QR جيرسو", "إنشاء وقراءة QR.",
        "مشاركة روابط سريعة.", "v0.1.0", "available", "#22c55e", "📱"),
    "inanna-tasks": ("مهام إينانا", "مدير المهام.",
        "قوائم وأولويات يومية.", "v0.1.0", "available", "#eab308", "✅"),
    "Ishtar-Backup": ("نسخ إشتار", "نسخ احتياطي.",
        "حفظ واستعادة البيانات.", "v0.1.0", "available", "#06b6d4", "💾"),
    "itura-offlinemaps": ("خرائط إيتورا", "خرائط دون اتصال.",
        "ملاحة بدون إنترنت.", "v0.1.0", "available", "#0ea5e9", "🗺️"),
    "kaspu-moneymanager": ("مدير مال كاسبو", "إدارة مالية شخصية.",
        "ميزانية ومصروفات.", "v0.1.0", "available", "#f59e0b", "💰"),
    "kish-scriptrun": ("مشغّل سكربت كيش", "تشغيل سكربتات.",
        "أتمتة مهام النظام.", "v0.1.0", "available", "#a855f7", "📜"),
    "kispu-painter": ("رسام كيسبو", "رسم وتحرير بكسل.",
        "لوحة فنية بسيطة.", "v0.1.0", "available", "#d946ef", "🎨"),
    "kubu-usbmanager": ("مدير USB كوبو", "إدارة أقراص USB.",
        "تركيب وفصل آمن.", "v0.1.0", "available", "#38bdf8", "🔌"),
    "lagash-keyboardshortcut": ("اختصارات لاغاش", "اختصارات لوحة المفاتيح.",
        "تخصيص اختصارات النظام.", "v0.1.0", "available", "#475569", "⌨️"),
    "lahmu-chess": ("شطرنج لحمو", "لعبة شطرنج.",
        "لعب محلي وتدريب.", "v0.1.0", "available", "#6366f1", "♟️"),
    "nabu-code": ("محرر نابو", "محرر أكواد خفيف.",
        "كتابة برامج سومر.", "v0.1.0", "available", "#007acc", "💻"),
    "namzu-programspremtion": ("ترويج نامزو", "عرض البرامج المميزة.",
        "اكتشاف تطبيقات سومر.", "v0.1.0", "available", "#ec4899", "⭐"),
    "rimush-bluetoothmanager": ("بلوتوث ريموش", "إدارة البلوتوث.",
        "اقتران أجهزة لاسلكية.", "v0.1.0", "available", "#3b82f6", "🔵"),
    "Scroll-PDF": ("مستعرض Scroll PDF", "قارئ PDF.",
        "عرض وثائق بسلاسة.", "v0.1.0", "available", "#ef4444", "📄"),
    "Shamash-taskmanager": ("مدير مهام شمش", "مراقبة العمليات.",
        "CPU و RAM والعمليات.", "v0.1.0", "available", "#10b981", "📊"),
    "sippar-mail": ("بريد سيبار", "عميل بريد.",
        "رسائل ومجلدات.", "v0.1.0", "available", "#0ea5e9", "✉️"),
    "ubara-reboot-shutdown": ("إعادة تشغيل أوبارا", "إيقاف وإعادة تشغيل.",
        "تحكم بطاقة الطاقة.", "v0.1.0", "available", "#f43f5e", "🔁"),
    "ursag-sysstartpage": ("صفحة بدء أورساج", "شاشة بدء النظام.",
        "واجهة ترحيبية.", "v0.1.0", "available", "#8b5cf6", "🏠"),
    "uruk-chat": ("دردشة أوروك", "مراسلة فورية.",
        "محادثات محلية.", "v0.1.0", "available", "#22c55e", "💬"),
    "zami-soundmixer": ("مزج صوت زامي", "تحكم بالصوت.",
        "مستويات وقنوات الصوت.", "v0.1.0", "available", "#06b6d4", "🔊"),
    "Alkem-Converter": ("محوّل ألكيم", "تحويل صيغ الملفات.",
        "وحدات قياس وملفات.", "v0.1.0", "available", "#f97316", "🔄"),
    "Ziusudra-Compress": ("ضغط زيوسودرا", "أرشفة وضغط.",
        "ملفات مضغوطة بأمان.", "v0.1.0", "available", "#64748b", "🗜️"),
}

ICON_MAP = {
    "download": "⬇️", "router": "📡", "mp4": "🎬", "convert": "🔄", "lock": "🔒",
    "cast": "📡", "game": "🕹️", "startup": "🚀", "file": "📁", "password": "🔐",
    "firewall": "🛡️", "share": "🔗", "editor": "✏️", "clean": "🧹", "qr": "📱",
    "setting": "⚙️", "task": "✅", "backup": "💾", "map": "🗺️", "money": "💰",
    "script": "📜", "paint": "🎨", "usb": "🔌", "keyboard": "⌨️", "chess": "♟️",
    "search": "🔍", "phone": "📱", "clipboard": "📋", "network": "📶", "code": "💻",
    "tv": "📺", "extension": "🧩", "browser": "🌐", "screenshot": "📸",
    "notification": "🔔", "archive": "📦", "bluetooth": "🔵", "ai": "🔮",
    "pdf": "📄", "mail": "✉️", "shell": "🐚", "reboot": "🔁", "desktop": "🏛️",
    "chat": "💬", "sound": "🔊", "store": "🏪", "compress": "🗜️", "print": "🖨️",
}

COLORS = [
    "#6366f1", "#38bdf8", "#3b82f6", "#10b981", "#f59e0b", "#8b5cf6", "#ec4899",
    "#ef4444", "#14b8a6", "#06b6d4", "#84cc16", "#f97316", "#d946ef", "#475569",
    "#0ea5e9", "#eab308", "#22c55e", "#a855f7", "#f43f5e", "#64748b",
]


def normalize_id(folder_name: str) -> str:
    return folder_name.strip()


def pick_icon(app_id: str) -> str:
    low = app_id.lower()
    for key, icon in ICON_MAP.items():
        if key in low:
            return icon
    return "📦"


def color_for(app_id: str) -> str:
    h = int(hashlib.md5(app_id.encode()).hexdigest()[:8], 16)
    return COLORS[h % len(COLORS)]


def cargo_package_name(path: str) -> str:
    toml = os.path.join(path, "Cargo.toml")
    if not os.path.isfile(toml):
        return normalize_id(os.path.basename(path))
    with open(toml, encoding="utf-8") as f:
        for line in f:
            m = re.match(r'^name\s*=\s*"(.+)"', line.strip())
            if m:
                return m.group(1)
    return normalize_id(os.path.basename(path))


def is_developed(path: str) -> bool:
    main = os.path.join(path, "src", "main.rs")
    if not os.path.isfile(main):
        return False
    with open(main, encoding="utf-8") as f:
        lines = [ln for ln in f.readlines() if ln.strip()]
    return len(lines) > 10


def is_built(path: str, pkg: str) -> bool:
    folder = os.path.basename(path)
    checks = [
        os.path.join(path, pkg),
        os.path.join(path, folder),
        os.path.join(path, "target-alpine", "release", pkg),
        os.path.join(path, "target", "debug", pkg),
        f"/usr/bin/{pkg}",
    ]
    return any(os.path.isfile(p) and os.access(p, os.X_OK) for p in checks)


def app_status(path: str, pkg: str) -> str:
    return "installed" if is_built(path, pkg) or is_developed(path) else "available"


def slint_escape(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def entry(folder_name: str) -> str:
    path = os.path.join(PROGRAMS, folder_name)
    app_id = normalize_id(folder_name)
    pkg = cargo_package_name(path)

    if app_id in KNOWN:
        name, desc, long_d, ver, status, color, icon = KNOWN[app_id]
        if status == "available" and app_status(path, pkg) == "installed":
            status = "installed"
    else:
        en = re.sub(r"[-_]+", " ", app_id).title()
        name = f"{en} ({app_id})"
        desc = f"برنامج سومر OS — {en}."
        long_d = f"تطبيق {en} ضمن حزمة برامج نظام سومر OS."
        ver = "v0.1.0"
        status = app_status(path, pkg)
        color = color_for(app_id)
        icon = pick_icon(app_id)

    return f"""        {{
            id: "{slint_escape(app_id)}",
            name: "{slint_escape(name)}",
            category: "system",
            icon: "{icon}",
            description: "{slint_escape(desc)}",
            long_description: "{slint_escape(long_d)}",
            author: "مطور Sumer OS",
            version: "{ver}",
            status: "{status}",
            accent_color: {color},
            is_sumer: true,
        }}"""


def main() -> None:
    folders = sorted(
        n for n in os.listdir(PROGRAMS)
        if os.path.isdir(os.path.join(PROGRAMS, n))
    )
    sumer = ",\n".join(entry(f) for f in folders)
    ext = ",\n".join(
        f"""        {{
            id: "{r[0]}",
            name: "{slint_escape(r[1])}",
            category: "{r[2]}",
            icon: "{r[3]}",
            description: "{slint_escape(r[4])}",
            long_description: "{slint_escape(r[5])}",
            author: "{r[6]}",
            version: "{r[7]}",
            status: "{r[8]}",
            accent_color: {r[9]},
            is_sumer: false,
        }}"""
        for r in EXTERNAL
    )
    content = f"""import {{ AppInfo }} from "types.slint";

export global AppsData {{
    out property <[AppInfo]> default_apps: [
        // SUMER OS PROGRAMS ({len(folders)} apps)
{sumer},

        // GENERAL / EXTERNAL PROGRAMS
{ext}
    ];
}}
"""
    with open(OUT, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"Wrote {len(folders)} Sumer + {len(EXTERNAL)} external apps -> {OUT}")


if __name__ == "__main__":
    main()
