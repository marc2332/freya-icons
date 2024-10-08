use super::super::IconShape;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAArrowDown;
impl IconShape for LdAArrowDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.5 13h6" />
  <path d="m2 16 4.5-9 4.5 9" />
  <path d="M18 7v9" />
  <path d="m14 12 4 4 4-4" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAArrowUp;
impl IconShape for LdAArrowUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.5 13h6" />
  <path d="m2 16 4.5-9 4.5 9" />
  <path d="M18 16V7" />
  <path d="m14 11 4-4 4 4" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdALargeSmall;
impl IconShape for LdALargeSmall {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 14h-5" />
  <path d="M16 16v-3.5a2.5 2.5 0 0 1 5 0V16" />
  <path d="M4.5 13h6" />
  <path d="m3 16 4.5-9 4.5 9" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAccessibility;
impl IconShape for LdAccessibility {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="16" cy="4" r="1" />
  <path d="m18 19 1-7-6 1" />
  <path d="m5 8 3-3 5.5 3-2.36 3.5" />
  <path d="M4.24 14.5a5 5 0 0 0 6.88 6" />
  <path d="M13.76 17.5a5 5 0 0 0-6.88-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdActivity;
impl IconShape for LdActivity {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 12h-4l-3 9L9 3l-3 9H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAirVent;
impl IconShape for LdAirVent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 12H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
  <path d="M6 8h12" />
  <path d="M18.3 17.7a2.5 2.5 0 0 1-3.16 3.83 2.53 2.53 0 0 1-1.14-2V12" />
  <path d="M6.6 15.6A2 2 0 1 0 10 17v-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAirplay;
impl IconShape for LdAirplay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1" />
  <path d="m12 15 5 6H7Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlarmClockCheck;
impl IconShape for LdAlarmClockCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="13" r="8" />
  <path d="M5 3 2 6" />
  <path d="m22 6-3-3" />
  <path d="M6.38 18.7 4 21" />
  <path d="M17.64 18.67 20 21" />
  <path d="m9 13 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlarmClockMinus;
impl IconShape for LdAlarmClockMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="13" r="8" />
  <path d="M5 3 2 6" />
  <path d="m22 6-3-3" />
  <path d="M6.38 18.7 4 21" />
  <path d="M17.64 18.67 20 21" />
  <path d="M9 13h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlarmClockOff;
impl IconShape for LdAlarmClockOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6.87 6.87a8 8 0 1 0 11.26 11.26" />
  <path d="M19.9 14.25a8 8 0 0 0-9.15-9.15" />
  <path d="m22 6-3-3" />
  <path d="M6.26 18.67 4 21" />
  <path d="m2 2 20 20" />
  <path d="M4 4 2 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlarmClockPlus;
impl IconShape for LdAlarmClockPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="13" r="8" />
  <path d="M5 3 2 6" />
  <path d="m22 6-3-3" />
  <path d="M6.38 18.7 4 21" />
  <path d="M17.64 18.67 20 21" />
  <path d="M12 10v6" />
  <path d="M9 13h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlarmClock;
impl IconShape for LdAlarmClock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="13" r="8" />
  <path d="M12 9v4l2 2" />
  <path d="M5 3 2 6" />
  <path d="m22 6-3-3" />
  <path d="M6.38 18.7 4 21" />
  <path d="M17.64 18.67 20 21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlarmSmoke;
impl IconShape for LdAlarmSmoke {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 8a2 2 0 0 1-2-2V3h20v3a2 2 0 0 1-2 2Z" />
  <path d="m19 8-.8 3c-.1.6-.6 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L5 8" />
  <path d="M16 21c0-2.5 2-2.5 2-5" />
  <path d="M11 21c0-2.5 2-2.5 2-5" />
  <path d="M6 21c0-2.5 2-2.5 2-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlbum;
impl IconShape for LdAlbum {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <polyline points="11 3 11 11 14 8 17 11 17 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignCenterHorizontal;
impl IconShape for LdAlignCenterHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12h20" />
  <path d="M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4" />
  <path d="M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4" />
  <path d="M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1" />
  <path d="M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignCenterVertical;
impl IconShape for LdAlignCenterVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v20" />
  <path d="M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4" />
  <path d="M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4" />
  <path d="M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1" />
  <path d="M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignCenter;
impl IconShape for LdAlignCenter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="21" x2="3" y1="6" y2="6" />
  <line x1="17" x2="7" y1="12" y2="12" />
  <line x1="19" x2="5" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignEndHorizontal;
impl IconShape for LdAlignEndHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6"  x="4" y="2" rx="2" />
  <rect width="6" height="9" x="14" y="9" rx="2" />
  <path d="M22 22H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignEndVertical;
impl IconShape for LdAlignEndVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="6" x="2" y="4" rx="2" />
  <rect width="9" height="6" x="9" y="14" rx="2" />
  <path d="M22 22V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalDistributeCenter;
impl IconShape for LdAlignHorizontalDistributeCenter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="4" y="5" rx="2" />
  <rect width="6" height="10" x="14" y="7" rx="2" />
  <path d="M17 22v-5" />
  <path d="M17 7V2" />
  <path d="M7 22v-3" />
  <path d="M7 5V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalDistributeEnd;
impl IconShape for LdAlignHorizontalDistributeEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="4" y="5" rx="2" />
  <rect width="6" height="10" x="14" y="7" rx="2" />
  <path d="M10 2v20" />
  <path d="M20 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalDistributeStart;
impl IconShape for LdAlignHorizontalDistributeStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="4" y="5" rx="2" />
  <rect width="6" height="10" x="14" y="7" rx="2" />
  <path d="M4 2v20" />
  <path d="M14 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalJustifyCenter;
impl IconShape for LdAlignHorizontalJustifyCenter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="2" y="5" rx="2" />
  <rect width="6" height="10" x="16" y="7" rx="2" />
  <path d="M12 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalJustifyEnd;
impl IconShape for LdAlignHorizontalJustifyEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="2" y="5" rx="2" />
  <rect width="6" height="10" x="12" y="7" rx="2" />
  <path d="M22 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalJustifyStart;
impl IconShape for LdAlignHorizontalJustifyStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="6" y="5" rx="2" />
  <rect width="6" height="10" x="16" y="7" rx="2" />
  <path d="M2 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalSpaceAround;
impl IconShape for LdAlignHorizontalSpaceAround {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="10" x="9" y="7" rx="2" />
  <path d="M4 22V2" />
  <path d="M20 22V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignHorizontalSpaceBetween;
impl IconShape for LdAlignHorizontalSpaceBetween {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="14" x="3" y="5" rx="2" />
  <rect width="6" height="10" x="15" y="7" rx="2" />
  <path d="M3 2v20" />
  <path d="M21 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignJustify;
impl IconShape for LdAlignJustify {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="3" x2="21" y1="6" y2="6" />
  <line x1="3" x2="21" y1="12" y2="12" />
  <line x1="3" x2="21" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignLeft;
impl IconShape for LdAlignLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="21" x2="3" y1="6" y2="6" />
  <line x1="15" x2="3" y1="12" y2="12" />
  <line x1="17" x2="3" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignRight;
impl IconShape for LdAlignRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="21" x2="3" y1="6" y2="6" />
  <line x1="21" x2="9" y1="12" y2="12" />
  <line x1="21" x2="7" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignStartHorizontal;
impl IconShape for LdAlignStartHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6"  x="4" y="6" rx="2" />
  <rect width="6" height="9" x="14" y="6" rx="2" />
  <path d="M22 2H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignStartVertical;
impl IconShape for LdAlignStartVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="9" height="6" x="6" y="14" rx="2" />
  <rect  height="6" x="6" y="4" rx="2" />
  <path d="M2 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalDistributeCenter;
impl IconShape for LdAlignVerticalDistributeCenter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 17h-3" />
  <path d="M22 7h-5" />
  <path d="M5 17H2" />
  <path d="M7 7H2" />
  <rect x="5" y="14" width="14" height="6" rx="2" />
  <rect x="7" y="4" width="10" height="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalDistributeEnd;
impl IconShape for LdAlignVerticalDistributeEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="6" x="5" y="14" rx="2" />
  <rect width="10" height="6" x="7" y="4" rx="2" />
  <path d="M2 20h20" />
  <path d="M2 10h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalDistributeStart;
impl IconShape for LdAlignVerticalDistributeStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="6" x="5" y="14" rx="2" />
  <rect width="10" height="6" x="7" y="4" rx="2" />
  <path d="M2 14h20" />
  <path d="M2 4h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalJustifyCenter;
impl IconShape for LdAlignVerticalJustifyCenter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="6" x="5" y="16" rx="2" />
  <rect width="10" height="6" x="7" y="2" rx="2" />
  <path d="M2 12h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalJustifyEnd;
impl IconShape for LdAlignVerticalJustifyEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="6" x="5" y="12" rx="2" />
  <rect width="10" height="6" x="7" y="2" rx="2" />
  <path d="M2 22h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalJustifyStart;
impl IconShape for LdAlignVerticalJustifyStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="6" x="5" y="16" rx="2" />
  <rect width="10" height="6" x="7" y="6" rx="2" />
  <path d="M2 2h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalSpaceAround;
impl IconShape for LdAlignVerticalSpaceAround {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="10" height="6" x="7" y="9" rx="2" />
  <path d="M22 20H2" />
  <path d="M22 4H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAlignVerticalSpaceBetween;
impl IconShape for LdAlignVerticalSpaceBetween {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="6" x="5" y="15" rx="2" />
  <rect width="10" height="6" x="7" y="3" rx="2" />
  <path d="M2 21h20" />
  <path d="M2 3h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAmbulance;
impl IconShape for LdAmbulance {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 10H6" />
  <path d="M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2" />
  <path
    d="M19 18h2a1 1 0 0 0 1-1v-3.28a1 1 0 0 0-.684-.948l-1.923-.641a1 1 0 0 1-.578-.502l-1.539-3.076A1 1 0 0 0 16.382 8H14" />
  <path d="M8 8v4" />
  <path d="M9 18h6" />
  <circle cx="17" cy="18" r="2" />
  <circle cx="7" cy="18" r="2" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAmpersand;
impl IconShape for LdAmpersand {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.5 12c0 4.4-3.6 8-8 8A4.5 4.5 0 0 1 5 15.5c0-6 8-4 8-8.5a3 3 0 1 0-6 0c0 3 2.5 8.5 12 13" />
  <path d="M16 12h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAmpersands;
impl IconShape for LdAmpersands {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5" />
  <path d="M22 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAnchor;
impl IconShape for LdAnchor {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22V8" />
  <path d="M5 12H2a10 10 0 0 0 20 0h-3" />
  <circle cx="12" cy="5" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAngry;
impl IconShape for LdAngry {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M16 16s-1.5-2-4-2-4 2-4 2" />
  <path d="M7.5 8 10 9" />
  <path d="m14 9 2.5-1" />
  <path d="M9 10h0" />
  <path d="M15 10h0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAnnoyed;
impl IconShape for LdAnnoyed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M8 15h8" />
  <path d="M8 9h2" />
  <path d="M14 9h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAntenna;
impl IconShape for LdAntenna {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12 7 2" />
  <path d="m7 12 5-10" />
  <path d="m12 12 5-10" />
  <path d="m17 12 5-10" />
  <path d="M4.5 7h15" />
  <path d="M12 16v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAnvil;
impl IconShape for LdAnvil {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 10H6a4 4 0 0 1-4-4 1 1 0 0 1 1-1h4" />
  <path d="M7 5a1 1 0 0 1 1-1h13a1 1 0 0 1 1 1 7 7 0 0 1-7 7H8a1 1 0 0 1-1-1z" />
  <path d="M9 12v5" />
  <path d="M15 12v5" />
  <path d="M5 20a3 3 0 0 1 3-3h8a3 3 0 0 1 3 3 1 1 0 0 1-1 1H6a1 1 0 0 1-1-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAperture;
impl IconShape for LdAperture {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m14.31 8 5.74 9.94" />
  <path d="M9.69 8h11.48" />
  <path d="m7.38 12 5.74-9.94" />
  <path d="M9.69 16 3.95 6.06" />
  <path d="M14.31 16H2.83" />
  <path d="m16.62 12-5.74 9.94" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAppWindowMac;
impl IconShape for LdAppWindowMac {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="M6 8h.01" />
  <path d="M10 8h.01" />
  <path d="M14 8h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAppWindow;
impl IconShape for LdAppWindow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="2" y="4" width="20"  rx="2" />
  <path d="M10 4v4" />
  <path d="M2 8h20" />
  <path d="M6 4v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdApple;
impl IconShape for LdApple {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 20.94c1.5 0 2.75 1.06 4 1.06 3 0 6-8 6-12.22A4.91 4.91 0 0 0 17 5c-2.22 0-4 1.44-5 2-1-.56-2.78-2-5-2a4.9 4.9 0 0 0-5 4.78C2 14 5 22 8 22c1.25 0 2.5-1.06 4-1.06Z" />
  <path d="M10 2c1 .5 2 2 2 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArchiveRestore;
impl IconShape for LdArchiveRestore {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="5" x="2" y="3" rx="1" />
  <path d="M4 8v11a2 2 0 0 0 2 2h2" />
  <path d="M20 8v11a2 2 0 0 1-2 2h-2" />
  <path d="m9 15 3-3 3 3" />
  <path d="M12 12v9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArchiveX;
impl IconShape for LdArchiveX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="5" x="2" y="3" rx="1" />
  <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" />
  <path d="m9.5 17 5-5" />
  <path d="m9.5 12 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArchive;
impl IconShape for LdArchive {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="5" x="2" y="3" rx="1" />
  <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" />
  <path d="M10 12h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAreaChart;
impl IconShape for LdAreaChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <path d="M7 12v5h12V8l-5 5-4-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArmchair;
impl IconShape for LdArmchair {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3" />
  <path d="M3 16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H7v-2a2 2 0 0 0-4 0Z" />
  <path d="M5 18v2" />
  <path d="M19 18v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigDownDash;
impl IconShape for LdArrowBigDownDash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 5H9" />
  <path d="M15 9v3h4l-7 7-7-7h4V9z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigDown;
impl IconShape for LdArrowBigDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 6v6h4l-7 7-7-7h4V6h6z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigLeftDash;
impl IconShape for LdArrowBigLeftDash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 15V9" />
  <path d="M15 15h-3v4l-7-7 7-7v4h3v6z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigLeft;
impl IconShape for LdArrowBigLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 15h-6v4l-7-7 7-7v4h6v6z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigRightDash;
impl IconShape for LdArrowBigRightDash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 9v6" />
  <path d="M9 9h3V5l7 7-7 7v-4H9V9z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigRight;
impl IconShape for LdArrowBigRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 9h6V5l7 7-7 7v-4H6V9z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigUpDash;
impl IconShape for LdArrowBigUpDash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 19h6" />
  <path d="M9 15v-3H5l7-7 7 7h-4v3H9z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowBigUp;
impl IconShape for LdArrowBigUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 18v-6H5l7-7 7 7h-4v6H9z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDown01;
impl IconShape for LdArrowDown01 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 20V4" />
  <rect x="15" y="4" width="4" height="6" ry="2" />
  <path d="M17 20v-6h-2" />
  <path d="M15 20h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDown10;
impl IconShape for LdArrowDown10 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 20V4" />
  <path d="M17 10V4h-2" />
  <path d="M15 10h4" />
  <rect x="15" y="14" width="4" height="6" ry="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownAZ;
impl IconShape for LdArrowDownAZ {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 20V4" />
  <path d="M20 8h-5" />
  <path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10" />
  <path d="M15 14h5l-5 6h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownFromLine;
impl IconShape for LdArrowDownFromLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 3H5" />
  <path d="M12 21V7" />
  <path d="m6 15 6 6 6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownLeft;
impl IconShape for LdArrowDownLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 7 7 17" />
  <path d="M17 17H7V7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownNarrowWide;
impl IconShape for LdArrowDownNarrowWide {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 20V4" />
  <path d="M11 4h4" />
  <path d="M11 8h7" />
  <path d="M11 12h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownRight;
impl IconShape for LdArrowDownRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 7 10 10" />
  <path d="M17 7v10H7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownToDot;
impl IconShape for LdArrowDownToDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v14" />
  <path d="m19 9-7 7-7-7" />
  <circle cx="12" cy="21" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownToLine;
impl IconShape for LdArrowDownToLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 17V3" />
  <path d="m6 11 6 6 6-6" />
  <path d="M19 21H5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownUp;
impl IconShape for LdArrowDownUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 20V4" />
  <path d="m21 8-4-4-4 4" />
  <path d="M17 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownWideNarrow;
impl IconShape for LdArrowDownWideNarrow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 20V4" />
  <path d="M11 4h10" />
  <path d="M11 8h7" />
  <path d="M11 12h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDownZA;
impl IconShape for LdArrowDownZA {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 16 4 4 4-4" />
  <path d="M7 4v16" />
  <path d="M15 4h5l-5 6h5" />
  <path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20" />
  <path d="M20 18h-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowDown;
impl IconShape for LdArrowDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5v14" />
  <path d="m19 12-7 7-7-7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowLeftFromLine;
impl IconShape for LdArrowLeftFromLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 6-6 6 6 6" />
  <path d="M3 12h14" />
  <path d="M21 19V5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowLeftRight;
impl IconShape for LdArrowLeftRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3 4 7l4 4" />
  <path d="M4 7h16" />
  <path d="m16 21 4-4-4-4" />
  <path d="M20 17H4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowLeftToLine;
impl IconShape for LdArrowLeftToLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 19V5" />
  <path d="m13 6-6 6 6 6" />
  <path d="M7 12h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowLeft;
impl IconShape for LdArrowLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 19-7-7 7-7" />
  <path d="M19 12H5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowRightFromLine;
impl IconShape for LdArrowRightFromLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 5v14" />
  <path d="M21 12H7" />
  <path d="m15 18 6-6-6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowRightLeft;
impl IconShape for LdArrowRightLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 3 4 4-4 4" />
  <path d="M20 7H4" />
  <path d="m8 21-4-4 4-4" />
  <path d="M4 17h16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowRightToLine;
impl IconShape for LdArrowRightToLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 12H3" />
  <path d="m11 18 6-6-6-6" />
  <path d="M21 5v14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowRight;
impl IconShape for LdArrowRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 12h14" />
  <path d="m12 5 7 7-7 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUp01;
impl IconShape for LdArrowUp01 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
  <rect x="15" y="4" width="4" height="6" ry="2" />
  <path d="M17 20v-6h-2" />
  <path d="M15 20h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUp10;
impl IconShape for LdArrowUp10 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
  <path d="M17 10V4h-2" />
  <path d="M15 10h4" />
  <rect x="15" y="14" width="4" height="6" ry="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpAZ;
impl IconShape for LdArrowUpAZ {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
  <path d="M20 8h-5" />
  <path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10" />
  <path d="M15 14h5l-5 6h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpDown;
impl IconShape for LdArrowUpDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21 16-4 4-4-4" />
  <path d="M17 20V4" />
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpFromDot;
impl IconShape for LdArrowUpFromDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5 9 7-7 7 7" />
  <path d="M12 16V2" />
  <circle cx="12" cy="21" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpFromLine;
impl IconShape for LdArrowUpFromLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18 9-6-6-6 6" />
  <path d="M12 3v14" />
  <path d="M5 21h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpLeft;
impl IconShape for LdArrowUpLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 17V7h10" />
  <path d="M17 17 7 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpNarrowWide;
impl IconShape for LdArrowUpNarrowWide {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
  <path d="M11 12h4" />
  <path d="M11 16h7" />
  <path d="M11 20h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpRight;
impl IconShape for LdArrowUpRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 7h10v10" />
  <path d="M7 17 17 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpToLine;
impl IconShape for LdArrowUpToLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 3h14" />
  <path d="m18 13-6-6-6 6" />
  <path d="M12 7v14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpWideNarrow;
impl IconShape for LdArrowUpWideNarrow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
  <path d="M11 12h10" />
  <path d="M11 16h7" />
  <path d="M11 20h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUpZA;
impl IconShape for LdArrowUpZA {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 8 4-4 4 4" />
  <path d="M7 4v16" />
  <path d="M15 4h5l-5 6h5" />
  <path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20" />
  <path d="M20 18h-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowUp;
impl IconShape for LdArrowUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5 12 7-7 7 7" />
  <path d="M12 19V5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdArrowsUpFromLine;
impl IconShape for LdArrowsUpFromLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m4 6 3-3 3 3" />
  <path d="M7 17V3" />
  <path d="m14 6 3-3 3 3" />
  <path d="M17 17V3" />
  <path d="M4 21h16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAsterisk;
impl IconShape for LdAsterisk {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 6v12" />
  <path d="M17.196 9 6.804 15" />
  <path d="m6.804 9 10.392 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAtSign;
impl IconShape for LdAtSign {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="4" />
  <path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAtom;
impl IconShape for LdAtom {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="1" />
  <path d="M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z" />
  <path d="M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAudioLines;
impl IconShape for LdAudioLines {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 10v3" />
  <path d="M6 6v11" />
  <path d="M10 3v18" />
  <path d="M14 8v7" />
  <path d="M18 5v13" />
  <path d="M22 10v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAudioWaveform;
impl IconShape for LdAudioWaveform {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 13a2 2 0 0 0 2-2V7a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0V4a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0v-4a2 2 0 0 1 2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAward;
impl IconShape for LdAward {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="8" r="6" />
  <path d="M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAxe;
impl IconShape for LdAxe {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m14 12-8.5 8.5a2.12 2.12 0 1 1-3-3L11 9" />
  <path d="M15 13 9 7l4-4 6 6h3a8 8 0 0 1-7 7z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdAxis3d;
impl IconShape for LdAxis3d {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4v16h16" />
  <path d="m4 20 7-7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBaby;
impl IconShape for LdBaby {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 12h.01" />
  <path d="M15 12h.01" />
  <path d="M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5" />
  <path d="M19 6.3a9 9 0 0 1 1.8 3.9 2 2 0 0 1 0 3.6 9 9 0 0 1-17.6 0 2 2 0 0 1 0-3.6A9 9 0 0 1 12 3c2 0 3.5 1.1 3.5 2.5s-.9 2.5-2 2.5c-.8 0-1.5-.4-1.5-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBackpack;
impl IconShape for LdBackpack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2Z" />
  <path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" />
  <path d="M8 21v-5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v5" />
  <path d="M8 10h8" />
  <path d="M8 18h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeAlert;
impl IconShape for LdBadgeAlert {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <line x1="12" x2="12" y1="8" y2="12" />
  <line x1="12" x2="12.01" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeCent;
impl IconShape for LdBadgeCent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M12 7v10" />
  <path d="M15.4 10a4 4 0 1 0 0 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeCheck;
impl IconShape for LdBadgeCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="m9 12 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeDollarSign;
impl IconShape for LdBadgeDollarSign {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />
  <path d="M12 18V6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeEuro;
impl IconShape for LdBadgeEuro {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M7 12h5" />
  <path d="M15 9.4a4 4 0 1 0 0 5.2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeHelp;
impl IconShape for LdBadgeHelp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
  <line x1="12" x2="12.01" y1="17" y2="17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeIndianRupee;
impl IconShape for LdBadgeIndianRupee {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M8 8h8" />
  <path d="M8 12h8" />
  <path d="m13 17-5-1h1a4 4 0 0 0 0-8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeInfo;
impl IconShape for LdBadgeInfo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <line x1="12" x2="12" y1="16" y2="12" />
  <line x1="12" x2="12.01" y1="8" y2="8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeJapaneseYen;
impl IconShape for LdBadgeJapaneseYen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="m9 8 3 3v7" />
  <path d="m12 11 3-3" />
  <path d="M9 12h6" />
  <path d="M9 16h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeMinus;
impl IconShape for LdBadgeMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <line x1="8" x2="16" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgePercent;
impl IconShape for LdBadgePercent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="m15 9-6 6" />
  <path d="M9 9h.01" />
  <path d="M15 15h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgePlus;
impl IconShape for LdBadgePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <line x1="12" x2="12" y1="8" y2="16" />
  <line x1="8" x2="16" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgePoundSterling;
impl IconShape for LdBadgePoundSterling {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M8 12h4" />
  <path d="M10 16V9.5a2.5 2.5 0 0 1 5 0" />
  <path d="M8 16h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeRussianRuble;
impl IconShape for LdBadgeRussianRuble {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M9 16h5" />
  <path d="M9 12h5a2 2 0 1 0 0-4h-3v9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeSwissFranc;
impl IconShape for LdBadgeSwissFranc {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <path d="M11 17V8h4" />
  <path d="M11 12h3" />
  <path d="M9 16h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadgeX;
impl IconShape for LdBadgeX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
  <line x1="15" x2="9" y1="9" y2="15" />
  <line x1="9" x2="15" y1="9" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBadge;
impl IconShape for LdBadge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBaggageClaim;
impl IconShape for LdBaggageClaim {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2" />
  <path d="M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10" />
  <rect width="13" height="8" x="8" y="6" rx="1" />
  <circle cx="18" cy="20" r="2" />
  <circle cx="9" cy="20" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBan;
impl IconShape for LdBan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m4.9 4.9 14.2 14.2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBanana;
impl IconShape for LdBanana {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 13c3.5-2 8-2 10 2a5.5 5.5 0 0 1 8 5" />
  <path d="M5.15 17.89c5.52-1.52 8.65-6.89 7-12C11.55 4 11.5 2 13 2c3.22 0 5 5.5 5 8 0 6.5-4.2 12-10.49 12C5.11 22 2 22 2 20c0-1.5 1.14-1.55 3.15-2.11Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBanknote;
impl IconShape for LdBanknote {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="12" x="2" y="6" rx="2" />
  <circle cx="12" cy="12" r="2" />
  <path d="M6 12h.01M18 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChart2;
impl IconShape for LdBarChart2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="18" x2="18" y1="20" y2="10" />
  <line x1="12" x2="12" y1="20" y2="4" />
  <line x1="6" x2="6" y1="20" y2="14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChart3;
impl IconShape for LdBarChart3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <path d="M18 17V9" />
  <path d="M13 17V5" />
  <path d="M8 17v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChart4;
impl IconShape for LdBarChart4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <path d="M13 17V9" />
  <path d="M18 17V5" />
  <path d="M8 17v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChartBig;
impl IconShape for LdBarChartBig {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <rect width="4" height="7" x="7" y="10" rx="1" />
  <rect width="4" height="12" x="15" y="5" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChartHorizontalBig;
impl IconShape for LdBarChartHorizontalBig {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <rect width="12" height="4" x="7" y="5" rx="1" />
  <rect width="7" height="4" x="7" y="13" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChartHorizontal;
impl IconShape for LdBarChartHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <path d="M7 16h8" />
  <path d="M7 11h12" />
  <path d="M7 6h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarChart;
impl IconShape for LdBarChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="12" y1="20" y2="10" />
  <line x1="18" x2="18" y1="20" y2="4" />
  <line x1="6" x2="6" y1="20" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBarcode;
impl IconShape for LdBarcode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 5v14" />
  <path d="M8 5v14" />
  <path d="M12 5v14" />
  <path d="M17 5v14" />
  <path d="M21 5v14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBaseline;
impl IconShape for LdBaseline {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 20h16" />
  <path d="m6 16 6-12 6 12" />
  <path d="M8 12h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBath;
impl IconShape for LdBath {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 6 6.5 3.5a1.5 1.5 0 0 0-1-.5C4.683 3 4 3.683 4 4.5V17a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" />
  <line x1="10" x2="8" y1="5" y2="7" />
  <line x1="2" x2="22" y1="12" y2="12" />
  <line x1="7" x2="7" y1="19" y2="21" />
  <line x1="17" x2="17" y1="19" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBatteryCharging;
impl IconShape for LdBatteryCharging {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2" />
  <path d="M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1" />
  <path d="m11 7-3 5h4l-3 5" />
  <line x1="22" x2="22" y1="11" y2="13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBatteryFull;
impl IconShape for LdBatteryFull {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="10" x="2" y="7" rx="2" ry="2" />
  <line x1="22" x2="22" y1="11" y2="13" />
  <line x1="6" x2="6" y1="11" y2="13" />
  <line x1="10" x2="10" y1="11" y2="13" />
  <line x1="14" x2="14" y1="11" y2="13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBatteryLow;
impl IconShape for LdBatteryLow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="10" x="2" y="7" rx="2" ry="2" />
  <line x1="22" x2="22" y1="11" y2="13" />
  <line x1="6" x2="6" y1="11" y2="13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBatteryMedium;
impl IconShape for LdBatteryMedium {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="10" x="2" y="7" rx="2" ry="2" />
  <line x1="22" x2="22" y1="11" y2="13" />
  <line x1="6" x2="6" y1="11" y2="13" />
  <line x1="10" x2="10" y1="11" y2="13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBatteryWarning;
impl IconShape for LdBatteryWarning {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2" />
  <path d="M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2" />
  <line x1="22" x2="22" y1="11" y2="13" />
  <line x1="10" x2="10" y1="7" y2="13" />
  <line x1="10" x2="10" y1="17" y2="17.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBattery;
impl IconShape for LdBattery {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="10" x="2" y="7" rx="2" ry="2" />
  <line x1="22" x2="22" y1="11" y2="13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBeaker;
impl IconShape for LdBeaker {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4.5 3h15" />
  <path d="M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3" />
  <path d="M6 14h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBeanOff;
impl IconShape for LdBeanOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22a13.96 13.96 0 0 0 9.9-4.1" />
  <path d="M10.75 5.093A6 6 0 0 1 22 8c0 2.411-.61 4.68-1.683 6.66" />
  <path d="M5.341 10.62a4 4 0 0 0 6.487 1.208M10.62 5.341a4.015 4.015 0 0 1 2.039 2.04" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBean;
impl IconShape for LdBean {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.165 6.598C9.954 7.478 9.64 8.36 9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22c7.732 0 14-6.268 14-14a6 6 0 0 0-11.835-1.402Z" />
  <path d="M5.341 10.62a4 4 0 1 0 5.279-5.28" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBedDouble;
impl IconShape for LdBedDouble {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8" />
  <path d="M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" />
  <path d="M12 4v6" />
  <path d="M2 18h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBedSingle;
impl IconShape for LdBedSingle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8" />
  <path d="M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4" />
  <path d="M3 18h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBed;
impl IconShape for LdBed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 4v16" />
  <path d="M2 8h18a2 2 0 0 1 2 2v10" />
  <path d="M2 17h20" />
  <path d="M6 8v9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBeef;
impl IconShape for LdBeef {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12.5" cy="8.5" r="2.5" />
  <path d="M12.5 2a6.5 6.5 0 0 0-6.22 4.6c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c4 0 8.4-1.8 11.4-4.3A6.5 6.5 0 0 0 12.5 2Z" />
  <path d="m18.5 6 2.19 4.5a6.48 6.48 0 0 1 .31 2 6.49 6.49 0 0 1-2.6 5.2C15.4 20.2 11 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBeerOff;
impl IconShape for LdBeerOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 13v5" />
  <path d="M17 11.47V8" />
  <path d="M17 11h1a3 3 0 0 1 2.745 4.211" />
  <path d="m2 2 20 20" />
  <path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-3" />
  <path d="M7.536 7.535C6.766 7.649 6.154 8 5.5 8a2.5 2.5 0 0 1-1.768-4.268" />
  <path d="M8.727 3.204C9.306 2.767 9.885 2 11 2c1.56 0 2 1.5 3 1.5s1.72-.5 2.5-.5a1 1 0 1 1 0 5c-.78 0-1.5-.5-2.5-.5a3.149 3.149 0 0 0-.842.12" />
  <path d="M9 14.6V18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBeer;
impl IconShape for LdBeer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 11h1a3 3 0 0 1 0 6h-1" />
  <path d="M9 12v6" />
  <path d="M13 12v6" />
  <path d="M14 7.5c-1 0-1.44.5-3 .5s-2-.5-3-.5-1.72.5-2.5.5a2.5 2.5 0 0 1 0-5c.78 0 1.57.5 2.5.5S9.44 2 11 2s2 1.5 3 1.5 1.72-.5 2.5-.5a2.5 2.5 0 0 1 0 5c-.78 0-1.5-.5-2.5-.5Z" />
  <path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBellDot;
impl IconShape for LdBellDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19.4 14.9C20.2 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 .7 0 1.3.1 1.9.3" />
  <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
  <circle cx="18" cy="8" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBellElectric;
impl IconShape for LdBellElectric {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.8 4A6.3 8.7 0 0 1 20 9" />
  <path d="M9 9h.01" />
  <circle cx="9" cy="9" r="7" />
  <rect width="10" height="6" x="4" y="16" rx="2" />
  <path d="M14 19c3 0 4.6-1.6 4.6-1.6" />
  <circle cx="20" cy="16" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBellMinus;
impl IconShape for LdBellMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.4 12c.8 3.8 2.6 5 2.6 5H3s3-2 3-9c0-3.3 2.7-6 6-6 1.8 0 3.4.8 4.5 2" />
  <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
  <path d="M15 8h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBellOff;
impl IconShape for LdBellOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.7 3A6 6 0 0 1 18 8a21.3 21.3 0 0 0 .6 5" />
  <path d="M17 17H3s3-2 3-9a4.67 4.67 0 0 1 .3-1.7" />
  <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBellPlus;
impl IconShape for LdBellPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19.3 14.8C20.1 16.4 21 17 21 17H3s3-2 3-9c0-3.3 2.7-6 6-6 1 0 1.9.2 2.8.7" />
  <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
  <path d="M15 8h6" />
  <path d="M18 5v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBellRing;
impl IconShape for LdBellRing {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
  <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
  <path d="M4 2C2.8 3.7 2 5.7 2 8" />
  <path d="M22 8c0-2.3-.8-4.3-2-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBell;
impl IconShape for LdBell {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
  <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBetweenHorizontalEnd;
impl IconShape for LdBetweenHorizontalEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="13" height="7" x="3" y="3" rx="1" />
  <path d="m22 15-3-3 3-3" />
  <rect width="13" height="7" x="3" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBetweenHorizontalStart;
impl IconShape for LdBetweenHorizontalStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="13" height="7" x="8" y="3" rx="1" />
  <path d="m2 9 3 3-3 3" />
  <rect width="13" height="7" x="8" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBetweenVerticalEnd;
impl IconShape for LdBetweenVerticalEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="13" x="3" y="3" rx="1" />
  <path d="m9 22 3-3 3 3" />
  <rect width="7" height="13" x="14" y="3" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBetweenVerticalStart;
impl IconShape for LdBetweenVerticalStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="13" x="3" y="8" rx="1" />
  <path d="m15 2-3 3-3-3" />
  <rect width="7" height="13" x="14" y="8" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBike;
impl IconShape for LdBike {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18.5" cy="17.5" r="3.5" />
  <circle cx="5.5" cy="17.5" r="3.5" />
  <circle cx="15" cy="5" r="1" />
  <path d="M12 17.5V14l-3-3 4-3 2 3h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBinary;
impl IconShape for LdBinary {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="14" y="14" width="4" height="6" rx="2" />
  <rect x="6" y="4" width="4" height="6" rx="2" />
  <path d="M6 20h4" />
  <path d="M14 10h4" />
  <path d="M6 14h2v6" />
  <path d="M14 4h2v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBiohazard;
impl IconShape for LdBiohazard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="11.9" r="2" />
  <path d="M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6" />
  <path d="m8.9 10.1 1.4.8" />
  <path d="M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5" />
  <path d="m15.1 10.1-1.4.8" />
  <path d="M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2" />
  <path d="M12 13.9v1.6" />
  <path d="M13.5 5.4c-1-.2-2-.2-3 0" />
  <path d="M17 16.4c.7-.7 1.2-1.6 1.5-2.5" />
  <path d="M5.5 13.9c.3.9.8 1.8 1.5 2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBird;
impl IconShape for LdBird {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 7h.01" />
  <path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20" />
  <path d="m20 7 2 .5-2 .5" />
  <path d="M10 18v3" />
  <path d="M14 17.75V21" />
  <path d="M7 18a6 6 0 0 0 3.84-10.61" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBitcoin;
impl IconShape for LdBitcoin {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11.767 19.089c4.924.868 6.14-6.025 1.216-6.894m-1.216 6.894L5.86 18.047m5.908 1.042-.347 1.97m1.563-8.864c4.924.869 6.14-6.025 1.215-6.893m-1.215 6.893-3.94-.694m5.155-6.2L8.29 4.26m5.908 1.042.348-1.97M7.48 20.364l3.126-17.727" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBlend;
impl IconShape for LdBlend {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="9" cy="9" r="7" />
  <circle cx="15" cy="15" r="7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBlinds;
impl IconShape for LdBlinds {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3h18" />
  <path d="M20 7H8" />
  <path d="M20 11H8" />
  <path d="M10 19h10" />
  <path d="M8 15h12" />
  <path d="M4 3v14" />
  <circle cx="4" cy="19" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBlocks;
impl IconShape for LdBlocks {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="7" x="14" y="3" rx="1" />
  <path d="M10 21V8a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-5a1 1 0 0 0-1-1H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBluetoothConnected;
impl IconShape for LdBluetoothConnected {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 7 10 10-5 5V2l5 5L7 17" />
  <line x1="18" x2="21" y1="12" y2="12" />
  <line x1="3" x2="6" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBluetoothOff;
impl IconShape for LdBluetoothOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 17-5 5V12l-5 5" />
  <path d="m2 2 20 20" />
  <path d="M14.5 9.5 17 7l-5-5v4.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBluetoothSearching;
impl IconShape for LdBluetoothSearching {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 7 10 10-5 5V2l5 5L7 17" />
  <path d="M20.83 14.83a4 4 0 0 0 0-5.66" />
  <path d="M18 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBluetooth;
impl IconShape for LdBluetooth {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 7 10 10-5 5V2l5 5L7 17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBold;
impl IconShape for LdBold {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 12a4 4 0 0 0 0-8H6v8" />
  <path d="M15 20a4 4 0 0 0 0-8H6v8Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBolt;
impl IconShape for LdBolt {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
  <circle cx="12" cy="12" r="4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBomb;
impl IconShape for LdBomb {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11" cy="13" r="9" />
  <path d="M14.35 4.65 16.3 2.7a2.41 2.41 0 0 1 3.4 0l1.6 1.6a2.4 2.4 0 0 1 0 3.4l-1.95 1.95" />
  <path d="m22 2-1.5 1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBone;
impl IconShape for LdBone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookA;
impl IconShape for LdBookA {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="m8 13 4-7 4 7" />
  <path d="M9.1 11h5.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookAudio;
impl IconShape for LdBookAudio {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M8 8v3" />
  <path d="M12 6v7" />
  <path d="M16 8v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookCheck;
impl IconShape for LdBookCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="m9 9.5 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookCopy;
impl IconShape for LdBookCopy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 16V4a2 2 0 0 1 2-2h11" />
  <path d="M5 14H4a2 2 0 1 0 0 4h1" />
  <path d="M22 18H11a2 2 0 1 0 0 4h11V6H11a2 2 0 0 0-2 2v12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookDashed;
impl IconShape for LdBookDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 22h-2" />
  <path d="M20 15v2h-2" />
  <path d="M4 19.5V15" />
  <path d="M20 8v3" />
  <path d="M18 2h2v2" />
  <path d="M4 11V9" />
  <path d="M12 2h2" />
  <path d="M12 22h2" />
  <path d="M12 17h2" />
  <path d="M8 22H6.5a2.5 2.5 0 0 1 0-5H8" />
  <path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookDown;
impl IconShape for LdBookDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M12 13V7" />
  <path d="m9 10 3 3 3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookHeadphones;
impl IconShape for LdBookHeadphones {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <circle cx="9" cy="12" r="1" />
  <path d="M8 12v-2a4 4 0 0 1 8 0v2" />
  <circle cx="15" cy="12" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookHeart;
impl IconShape for LdBookHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M16 8.2C16 7 15 6 13.8 6c-.8 0-1.4.3-1.8.9-.4-.6-1-.9-1.8-.9C9 6 8 7 8 8.2c0 .6.3 1.2.7 1.6h0C10 11.1 12 13 12 13s2-1.9 3.3-3.1h0c.4-.4.7-1 .7-1.7z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookImage;
impl IconShape for LdBookImage {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <circle cx="10" cy="8" r="2" />
  <path d="m20 13.7-2.1-2.1c-.8-.8-2-.8-2.8 0L9.7 17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookKey;
impl IconShape for LdBookKey {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H14" />
  <path d="M20 8v14H6.5a2.5 2.5 0 0 1 0-5H20" />
  <circle cx="14" cy="8" r="2" />
  <path d="m20 2-4.5 4.5" />
  <path d="m19 3 1 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookLock;
impl IconShape for LdBookLock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10" />
  <path d="M20 15v7H6.5a2.5 2.5 0 0 1 0-5H20" />
  <rect width="8" height="5" x="12" y="6" rx="1" />
  <path d="M18 6V4a2 2 0 1 0-4 0v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookMarked;
impl IconShape for LdBookMarked {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <polyline points="10 2 10 10 13 7 16 10 16 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookMinus;
impl IconShape for LdBookMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M9 10h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookOpenCheck;
impl IconShape for LdBookOpenCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3H2v15h7c1.7 0 3 1.3 3 3V7c0-2.2-1.8-4-4-4Z" />
  <path d="m16 12 2 2 4-4" />
  <path d="M22 6V3h-6c-2.2 0-4 1.8-4 4v14c0-1.7 1.3-3 3-3h7v-2.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookOpenText;
impl IconShape for LdBookOpenText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
  <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />
  <path d="M6 8h2" />
  <path d="M6 12h2" />
  <path d="M16 8h2" />
  <path d="M16 12h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookOpen;
impl IconShape for LdBookOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" />
  <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookPlus;
impl IconShape for LdBookPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M9 10h6" />
  <path d="M12 7v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookText;
impl IconShape for LdBookText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M8 7h6" />
  <path d="M8 11h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookType;
impl IconShape for LdBookType {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M16 8V6H8v2" />
  <path d="M12 6v7" />
  <path d="M10 13h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookUp2;
impl IconShape for LdBookUp2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2" />
  <path d="M18 2h2v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M12 13V7" />
  <path d="m9 10 3-3 3 3" />
  <path d="m9 5 3-3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookUp;
impl IconShape for LdBookUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="M12 13V7" />
  <path d="m9 10 3-3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookUser;
impl IconShape for LdBookUser {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <circle cx="12" cy="8" r="2" />
  <path d="M15 13a3 3 0 1 0-6 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookX;
impl IconShape for LdBookX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
  <path d="m14.5 7-5 5" />
  <path d="m9.5 7 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBook;
impl IconShape for LdBook {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookmarkCheck;
impl IconShape for LdBookmarkCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2Z" />
  <path d="m9 10 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookmarkMinus;
impl IconShape for LdBookmarkMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />
  <line x1="15" x2="9" y1="10" y2="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookmarkPlus;
impl IconShape for LdBookmarkPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />
  <line x1="12" x2="12" y1="7" y2="13" />
  <line x1="15" x2="9" y1="10" y2="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookmarkX;
impl IconShape for LdBookmarkX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2Z" />
  <path d="m14.5 7.5-5 5" />
  <path d="m9.5 7.5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBookmark;
impl IconShape for LdBookmark {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBoomBox;
impl IconShape for LdBoomBox {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" />
  <path d="M8 8v1" />
  <path d="M12 8v1" />
  <path d="M16 8v1" />
  <rect width="20" height="12" x="2" y="9" rx="2" />
  <circle cx="8" cy="15" r="2" />
  <circle cx="16" cy="15" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBotMessageSquare;
impl IconShape for LdBotMessageSquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 6V2H8" />
  <path d="m8 18-4 4V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2Z" />
  <path d="M2 12h2" />
  <path d="M9 11v2" />
  <path d="M15 11v2" />
  <path d="M20 12h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBotOff;
impl IconShape for LdBotOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.67 8H18a2 2 0 0 1 2 2v4.33" />
  <path d="M2 14h2" />
  <path d="M20 14h2" />
  <path d="M22 22 2 2" />
  <path d="M8 8H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 1.414-.586" />
  <path d="M9 13v2" />
  <path d="M9.67 4H12v2.33" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBot;
impl IconShape for LdBot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 8V4H8" />
  <rect  height="12" x="4" y="8" rx="2" />
  <path d="M2 14h2" />
  <path d="M20 14h2" />
  <path d="M15 13v2" />
  <path d="M9 13v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBoxSelect;
impl IconShape for LdBoxSelect {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 3a2 2 0 0 0-2 2" />
  <path d="M19 3a2 2 0 0 1 2 2" />
  <path d="M21 19a2 2 0 0 1-2 2" />
  <path d="M5 21a2 2 0 0 1-2-2" />
  <path d="M9 3h1" />
  <path d="M9 21h1" />
  <path d="M14 3h1" />
  <path d="M14 21h1" />
  <path d="M3 9v1" />
  <path d="M21 9v1" />
  <path d="M3 14v1" />
  <path d="M21 14v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBox;
impl IconShape for LdBox {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" />
  <path d="m3.3 7 8.7 5 8.7-5" />
  <path d="M12 22V12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBoxes;
impl IconShape for LdBoxes {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z" />
  <path d="m7 16.5-4.74-2.85" />
  <path d="m7 16.5 5-3" />
  <path d="M7 16.5v5.17" />
  <path d="M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z" />
  <path d="m17 16.5-5-3" />
  <path d="m17 16.5 4.74-2.85" />
  <path d="M17 16.5v5.17" />
  <path d="M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z" />
  <path d="M12 8 7.26 5.15" />
  <path d="m12 8 4.74-2.85" />
  <path d="M12 13.5V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBraces;
impl IconShape for LdBraces {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1" />
  <path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBrackets;
impl IconShape for LdBrackets {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 3h3v18h-3" />
  <path d="M8 21H5V3h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBrainCircuit;
impl IconShape for LdBrainCircuit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z" />
  <path d="M9 13a4.5 4.5 0 0 0 3-4" />
  <path d="M6.003 5.125A3 3 0 0 0 6.401 6.5" />
  <path d="M3.477 10.896a4 4 0 0 1 .585-.396" />
  <path d="M6 18a4 4 0 0 1-1.967-.516" />
  <path d="M12 13h4" />
  <path d="M12 18h6a2 2 0 0 1 2 2v1" />
  <path d="M12 8h8" />
  <path d="M16 8V5a2 2 0 0 1 2-2" />
  <circle cx="16" cy="13" r=".5" />
  <circle cx="18" cy="3" r=".5" />
  <circle cx="20" cy="21" r=".5" />
  <circle cx="20" cy="8" r=".5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBrainCog;
impl IconShape for LdBrainCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5a3 3 0 1 0-5.997.142 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588 4 4 0 0 0 7.636 2.106 3.2 3.2 0 0 0 .164-.546c.028-.13.306-.13.335 0a3.2 3.2 0 0 0 .163.546 4 4 0 0 0 7.636-2.106 4 4 0 0 0 .556-6.588 4 4 0 0 0-2.526-5.77A3 3 0 1 0 12 5" />
  <path d="M17.599 6.5a3 3 0 0 0 .399-1.375" />
  <path d="M6.003 5.125A3 3 0 0 0 6.401 6.5" />
  <path d="M3.477 10.896a4 4 0 0 1 .585-.396" />
  <path d="M19.938 10.5a4 4 0 0 1 .585.396" />
  <path d="M6 18a4 4 0 0 1-1.967-.516" />
  <path d="M19.967 17.484A4 4 0 0 1 18 18" />
  <circle cx="12" cy="12" r="3" />
  <path d="m15.7 10.4-.9.4" />
  <path d="m9.2 13.2-.9.4" />
  <path d="m13.6 15.7-.4-.9" />
  <path d="m10.8 9.2-.4-.9" />
  <path d="m15.7 13.5-.9-.4" />
  <path d="m9.2 10.9-.9-.4" />
  <path d="m10.5 15.7.4-.9" />
  <path d="m13.1 9.2.4-.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBrain;
impl IconShape for LdBrain {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z" />
  <path d="M12 5a3 3 0 1 1 5.997.125 4 4 0 0 1 2.526 5.77 4 4 0 0 1-.556 6.588A4 4 0 1 1 12 18Z" />
  <path d="M15 13a4.5 4.5 0 0 1-3-4 4.5 4.5 0 0 1-3 4" />
  <path d="M17.599 6.5a3 3 0 0 0 .399-1.375" />
  <path d="M6.003 5.125A3 3 0 0 0 6.401 6.5" />
  <path d="M3.477 10.896a4 4 0 0 1 .585-.396" />
  <path d="M19.938 10.5a4 4 0 0 1 .585.396" />
  <path d="M6 18a4 4 0 0 1-1.967-.516" />
  <path d="M19.967 17.484A4 4 0 0 1 18 18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBrickWall;
impl IconShape for LdBrickWall {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M12 9v6" />
  <path d="M16 15v6" />
  <path d="M16 3v6" />
  <path d="M3 15h18" />
  <path d="M3 9h18" />
  <path d="M8 15v6" />
  <path d="M8 3v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBriefcaseBusiness;
impl IconShape for LdBriefcaseBusiness {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 12h.01" />
  <path d="M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" />
  <path d="M22 13a18.15 18.15 0 0 1-20 0" />
  <rect width="20" height="14" x="2" y="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBriefcaseMedical;
impl IconShape for LdBriefcaseMedical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 11v4" />
  <path d="M14 13h-4" />
  <path d="M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" />
  <path d="M18 6v14" />
  <path d="M6 6v14" />
  <rect width="20" height="14" x="2" y="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBriefcase;
impl IconShape for LdBriefcase {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" />
  <rect width="20" height="14" x="2" y="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBringToFront;
impl IconShape for LdBringToFront {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="8" y="8" width="8" height="8" rx="2" />
  <path d="M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2" />
  <path d="M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBrush;
impl IconShape for LdBrush {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9.06 11.9 8.07-8.06a2.85 2.85 0 1 1 4.03 4.03l-8.06 8.08" />
  <path d="M7.07 14.94c-1.66 0-3 1.35-3 3.02 0 1.33-2.5 1.52-2 2.02 1.08 1.1 2.49 2.02 4 2.02 2.2 0 4-1.8 4-4.04a3.01 3.01 0 0 0-3-3.02z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBugOff;
impl IconShape for LdBugOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 7.13V6a3 3 0 0 0-5.14-2.1L8 2" />
  <path d="M14.12 3.88 16 2" />
  <path d="M22 13h-4v-2a4 4 0 0 0-4-4h-1.3" />
  <path d="M20.97 5c0 2.1-1.6 3.8-3.5 4" />
  <path d="m2 2 20 20" />
  <path d="M7.7 7.7A4 4 0 0 0 6 11v3a6 6 0 0 0 11.13 3.13" />
  <path d="M12 20v-8" />
  <path d="M6 13H2" />
  <path d="M3 21c0-2.1 1.7-3.9 3.8-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBugPlay;
impl IconShape for LdBugPlay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.765 21.522a.5.5 0 0 1-.765-.424v-8.196a.5.5 0 0 1 .765-.424l5.878 3.674a1 1 0 0 1 0 1.696z" />
  <path d="M14.12 3.88 16 2" />
  <path d="M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5" />
  <path d="M20.97 5c0 2.1-1.6 3.8-3.5 4" />
  <path d="M3 21c0-2.1 1.7-3.9 3.8-4" />
  <path d="M6 13H2" />
  <path d="M6.53 9C4.6 8.8 3 7.1 3 5" />
  <path d="m8 2 1.88 1.88" />
  <path d="M9 7.13v-1a3.003 3.003 0 1 1 6 0v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBug;
impl IconShape for LdBug {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8 2 1.88 1.88" />
  <path d="M14.12 3.88 16 2" />
  <path d="M9 7.13v-1a3.003 3.003 0 1 1 6 0v1" />
  <path d="M12 20c-3.3 0-6-2.7-6-6v-3a4 4 0 0 1 4-4h4a4 4 0 0 1 4 4v3c0 3.3-2.7 6-6 6" />
  <path d="M12 20v-9" />
  <path d="M6.53 9C4.6 8.8 3 7.1 3 5" />
  <path d="M6 13H2" />
  <path d="M3 21c0-2.1 1.7-3.9 3.8-4" />
  <path d="M20.97 5c0 2.1-1.6 3.8-3.5 4" />
  <path d="M22 13h-4" />
  <path d="M17.2 17c2.1.1 3.8 1.9 3.8 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBuilding2;
impl IconShape for LdBuilding2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z" />
  <path d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2" />
  <path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2" />
  <path d="M10 6h4" />
  <path d="M10 10h4" />
  <path d="M10 14h4" />
  <path d="M10 18h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBuilding;
impl IconShape for LdBuilding {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="20" x="4" y="2" rx="2" ry="2" />
  <path d="M9 22v-4h6v4" />
  <path d="M8 6h.01" />
  <path d="M16 6h.01" />
  <path d="M12 6h.01" />
  <path d="M12 10h.01" />
  <path d="M12 14h.01" />
  <path d="M16 10h.01" />
  <path d="M16 14h.01" />
  <path d="M8 10h.01" />
  <path d="M8 14h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBusFront;
impl IconShape for LdBusFront {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 6 2 7" />
  <path d="M10 6h4" />
  <path d="m22 7-2-1" />
  <rect   x="4" y="3" rx="2" />
  <path d="M4 11h16" />
  <path d="M8 15h.01" />
  <path d="M16 15h.01" />
  <path d="M6 19v2" />
  <path d="M18 21v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdBus;
impl IconShape for LdBus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 6v6" />
  <path d="M15 6v6" />
  <path d="M2 12h19.6" />
  <path d="M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3" />
  <circle cx="7" cy="18" r="2" />
  <path d="M9 18h5" />
  <circle cx="16" cy="18" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCableCar;
impl IconShape for LdCableCar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 3h.01" />
  <path d="M14 2h.01" />
  <path d="m2 9 20-5" />
  <path d="M12 12V6.5" />
  <rect  height="10" x="4" y="12" rx="3" />
  <path d="M9 12v5" />
  <path d="M15 12v5" />
  <path d="M4 17h16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCable;
impl IconShape for LdCable {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 21v-2a1 1 0 0 1-1-1v-1a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1" />
  <path d="M19 15V6.5a1 1 0 0 0-7 0v11a1 1 0 0 1-7 0V9" />
  <path d="M21 21v-2h-4" />
  <path d="M3 5h4V3" />
  <path d="M7 5a1 1 0 0 1 1 1v1a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a1 1 0 0 1 1-1V3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCakeSlice;
impl IconShape for LdCakeSlice {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="9" cy="7" r="2" />
  <path d="M7.2 7.9 3 11v9c0 .6.4 1 1 1h16c.6 0 1-.4 1-1v-9c0-2-3-6-7-8l-3.6 2.6" />
  <path d="M16 13H3" />
  <path d="M16 17H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCake;
impl IconShape for LdCake {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8" />
  <path d="M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1" />
  <path d="M2 21h20" />
  <path d="M7 8v3" />
  <path d="M12 8v3" />
  <path d="M17 8v3" />
  <path d="M7 4h0.01" />
  <path d="M12 4h0.01" />
  <path d="M17 4h0.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalculator;
impl IconShape for LdCalculator {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="20" x="4" y="2" rx="2" />
  <line x1="8" x2="16" y1="6" y2="6" />
  <line x1="16" x2="16" y1="14" y2="18" />
  <path d="M16 10h.01" />
  <path d="M12 10h.01" />
  <path d="M8 10h.01" />
  <path d="M12 14h.01" />
  <path d="M8 14h.01" />
  <path d="M12 18h.01" />
  <path d="M8 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarCheck2;
impl IconShape for LdCalendarCheck2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <path d="M21 14V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
  <path d="M3 10h18" />
  <path d="m16 20 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarCheck;
impl IconShape for LdCalendarCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M3 10h18" />
  <path d="m9 16 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarClock;
impl IconShape for LdCalendarClock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 7.5V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.5" />
  <path d="M16 2v4" />
  <path d="M8 2v4" />
  <path d="M3 10h5" />
  <path d="M17.5 17.5 16 16.3V14" />
  <circle cx="16" cy="16" r="6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarDays;
impl IconShape for LdCalendarDays {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M3 10h18" />
  <path d="M8 14h.01" />
  <path d="M12 14h.01" />
  <path d="M16 14h.01" />
  <path d="M8 18h.01" />
  <path d="M12 18h.01" />
  <path d="M16 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarFold;
impl IconShape for LdCalendarFold {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <path d="M21 17V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h11Z" />
  <path d="M3 10h18" />
  <path d="M15 22v-4a2 2 0 0 1 2-2h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarHeart;
impl IconShape for LdCalendarHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 10h18V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7" />
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <path d="M21.29 14.7a2.43 2.43 0 0 0-2.65-.52c-.3.12-.57.3-.8.53l-.34.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L17.5 22l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarMinus2;
impl IconShape for LdCalendarMinus2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M3 10h18" />
  <path d="M10 16h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarMinus;
impl IconShape for LdCalendarMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
  <path d="M3 10h18" />
  <path d="M16 19h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarOff;
impl IconShape for LdCalendarOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4.2 4.2A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18" />
  <path d="M21 15.5V6a2 2 0 0 0-2-2H9.5" />
  <path d="M16 2v4" />
  <path d="M3 10h7" />
  <path d="M21 10h-5.5" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarPlus2;
impl IconShape for LdCalendarPlus2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M3 10h18" />
  <path d="M10 16h4" />
  <path d="M12 14v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarPlus;
impl IconShape for LdCalendarPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
  <path d="M3 10h18" />
  <path d="M16 19h6" />
  <path d="M19 16v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarRange;
impl IconShape for LdCalendarRange {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M16 2v4" />
  <path d="M3 10h18" />
  <path d="M8 2v4" />
  <path d="M17 14h-6" />
  <path d="M13 18H7" />
  <path d="M7 14h.01" />
  <path d="M17 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarSearch;
impl IconShape for LdCalendarSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.5" />
  <path d="M16 2v4" />
  <path d="M8 2v4" />
  <path d="M3 10h18" />
  <circle cx="18" cy="18" r="3" />
  <path d="m22 22-1.5-1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarX2;
impl IconShape for LdCalendarX2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" />
  <path d="M3 10h18" />
  <path d="m17 22 5-5" />
  <path d="m17 17 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendarX;
impl IconShape for LdCalendarX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M3 10h18" />
  <path d="m14 14-4 4" />
  <path d="m10 14 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCalendar;
impl IconShape for LdCalendar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M16 2v4" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <path d="M3 10h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCameraOff;
impl IconShape for LdCameraOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="22" y1="2" y2="22" />
  <path d="M7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16" />
  <path d="M9.5 4h5L17 7h3a2 2 0 0 1 2 2v7.5" />
  <path d="M14.121 15.121A3 3 0 1 1 9.88 10.88" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCamera;
impl IconShape for LdCamera {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z" />
  <circle cx="12" cy="13" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCandlestickChart;
impl IconShape for LdCandlestickChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 5v4" />
  <rect width="4" height="6" x="7" y="9" rx="1" />
  <path d="M9 15v2" />
  <path d="M17 3v2" />
  <rect width="4" height="8" x="15" y="5" rx="1" />
  <path d="M17 13v3" />
  <path d="M3 3v18h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCandyCane;
impl IconShape for LdCandyCane {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.464-2 2 2 0 1 0-3.464-2Z" />
  <path d="M17.75 7 15 2.1" />
  <path d="M10.9 4.8 13 9" />
  <path d="m7.9 9.7 2 4.4" />
  <path d="M4.9 14.7 7 18.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCandyOff;
impl IconShape for LdCandyOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8.5 8.5-1 1a4.95 4.95 0 0 0 7 7l1-1" />
  <path d="M11.843 6.187A4.947 4.947 0 0 1 16.5 7.5a4.947 4.947 0 0 1 1.313 4.657" />
  <path d="M14 16.5V14" />
  <path d="M14 6.5v1.843" />
  <path d="M10 10v7.5" />
  <path d="m16 7 1-5 1.367.683A3 3 0 0 0 19.708 3H21v1.292a3 3 0 0 0 .317 1.341L22 7l-5 1" />
  <path d="m8 17-1 5-1.367-.683A3 3 0 0 0 4.292 21H3v-1.292a3 3 0 0 0-.317-1.341L2 17l5-1" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCandy;
impl IconShape for LdCandy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9.5 7.5-2 2a4.95 4.95 0 1 0 7 7l2-2a4.95 4.95 0 1 0-7-7Z" />
  <path d="M14 6.5v10" />
  <path d="M10 7.5v10" />
  <path d="m16 7 1-5 1.37.68A3 3 0 0 0 19.7 3H21v1.3c0 .46.1.92.32 1.33L22 7l-5 1" />
  <path d="m8 17-1 5-1.37-.68A3 3 0 0 0 4.3 21H3v-1.3a3 3 0 0 0-.32-1.33L2 17l5-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCannabis;
impl IconShape for LdCannabis {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22v-4" />
  <path d="M7 12c-1.5 0-4.5 1.5-5 3 3.5 1.5 6 1 6 1-1.5 1.5-2 3.5-2 5 2.5 0 4.5-1.5 6-3 1.5 1.5 3.5 3 6 3 0-1.5-.5-3.5-2-5 0 0 2.5.5 6-1-.5-1.5-3.5-3-5-3 1.5-1 4-4 4-6-2.5 0-5.5 1.5-7 3 0-2.5-.5-5-2-7-1.5 2-2 4.5-2 7-1.5-1.5-4.5-3-7-3 0 2 2.5 5 4 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCaptionsOff;
impl IconShape for LdCaptionsOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.5 5H19a2 2 0 0 1 2 2v8.5" />
  <path d="M17 11h-.5" />
  <path d="M19 19H5a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2" />
  <path d="m2 2 20 20" />
  <path d="M7 11h4" />
  <path d="M7 15h2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCaptions;
impl IconShape for LdCaptions {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="14" x="3" y="5" rx="2" ry="2" />
  <path d="M7 15h4M15 15h2M7 11h2M13 11h4" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCarFront;
impl IconShape for LdCarFront {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8" />
  <path d="M7 14h.01" />
  <path d="M17 14h.01" />
  <rect width="18" height="8" x="3" y="10" rx="2" />
  <path d="M5 18v2" />
  <path d="M19 18v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCarTaxiFront;
impl IconShape for LdCarTaxiFront {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 2h4" />
  <path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8" />
  <path d="M7 14h.01" />
  <path d="M17 14h.01" />
  <rect width="18" height="8" x="3" y="10" rx="2" />
  <path d="M5 18v2" />
  <path d="M19 18v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCar;
impl IconShape for LdCar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2" />
  <circle cx="7" cy="17" r="2" />
  <path d="M9 17h6" />
  <circle cx="17" cy="17" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCaravan;
impl IconShape for LdCaravan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="4" height="4" x="2" y="9" />
  <rect width="4" height="10" x="10" y="9" />
  <path d="M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2" />
  <circle cx="8" cy="19" r="2" />
  <path d="M10 19h12v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCarrot;
impl IconShape for LdCarrot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.27 21.7s9.87-3.5 12.73-6.36a4.5 4.5 0 0 0-6.36-6.37C5.77 11.84 2.27 21.7 2.27 21.7zM8.64 14l-2.05-2.04M15.34 15l-2.46-2.46" />
  <path d="M22 9s-1.33-2-3.5-2C16.86 7 15 9 15 9s1.33 2 3.5 2S22 9 22 9z" />
  <path d="M15 2s-2 1.33-2 3.5S15 9 15 9s2-1.84 2-3.5C17 3.33 15 2 15 2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCaseLower;
impl IconShape for LdCaseLower {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="7" cy="12" r="3" />
  <path d="M10 9v6" />
  <circle cx="17" cy="12" r="3" />
  <path d="M14 7v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCaseSensitive;
impl IconShape for LdCaseSensitive {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 15 4-8 4 8" />
  <path d="M4 13h6" />
  <circle cx="18" cy="12" r="3" />
  <path d="M21 9v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCaseUpper;
impl IconShape for LdCaseUpper {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 15 4-8 4 8" />
  <path d="M4 13h6" />
  <path d="M15 11h4.5a2 2 0 0 1 0 4H15V7h4a2 2 0 0 1 0 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCassetteTape;
impl IconShape for LdCassetteTape {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <circle cx="8" cy="10" r="2" />
  <path d="M8 12h8" />
  <circle cx="16" cy="10" r="2" />
  <path d="m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCast;
impl IconShape for LdCast {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6" />
  <path d="M2 12a9 9 0 0 1 8 8" />
  <path d="M2 16a5 5 0 0 1 4 4" />
  <line x1="2" x2="2.01" y1="20" y2="20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCastle;
impl IconShape for LdCastle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 20v-9H2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z" />
  <path d="M18 11V4H6v7" />
  <path d="M15 22v-4a3 3 0 0 0-3-3v0a3 3 0 0 0-3 3v4" />
  <path d="M22 11V9" />
  <path d="M2 11V9" />
  <path d="M6 4V2" />
  <path d="M18 4V2" />
  <path d="M10 4V2" />
  <path d="M14 4V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCat;
impl IconShape for LdCat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5Z" />
  <path d="M8 14v.5" />
  <path d="M16 14v.5" />
  <path d="M11.25 16.25h1.5L12 17l-.75-.75Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCctv;
impl IconShape for LdCctv {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16.75 12h3.632a1 1 0 0 1 .894 1.447l-2.034 4.069a1 1 0 0 1-1.708.134l-2.124-2.97" />
  <path d="M17.106 9.053a1 1 0 0 1 .447 1.341l-3.106 6.211a1 1 0 0 1-1.342.447L3.61 12.3a2.92 2.92 0 0 1-1.3-3.91L3.69 5.6a2.92 2.92 0 0 1 3.92-1.3z" />
  <path d="M2 19h3.76a2 2 0 0 0 1.8-1.1L9 15" />
  <path d="M2 21v-4" />
  <path d="M7 9h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCheckCheck;
impl IconShape for LdCheckCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 6 7 17l-5-5" />
  <path d="m22 10-7.5 7.5L13 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCheck;
impl IconShape for LdCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 6 9 17l-5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChefHat;
impl IconShape for LdChefHat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 21a1 1 0 0 0 1-1v-5.35c0-.457.316-.844.727-1.041a4 4 0 0 0-2.134-7.589 5 5 0 0 0-9.186 0 4 4 0 0 0-2.134 7.588c.411.198.727.585.727 1.041V20a1 1 0 0 0 1 1Z" />
  <path d="M6 17h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCherry;
impl IconShape for LdCherry {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z" />
  <path d="M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z" />
  <path d="M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12" />
  <path d="M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronDown;
impl IconShape for LdChevronDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 9 6 6 6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronFirst;
impl IconShape for LdChevronFirst {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 18-6-6 6-6" />
  <path d="M7 6v12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronLast;
impl IconShape for LdChevronLast {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 18 6-6-6-6" />
  <path d="M17 6v12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronLeft;
impl IconShape for LdChevronLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 18-6-6 6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronRight;
impl IconShape for LdChevronRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 18 6-6-6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronUp;
impl IconShape for LdChevronUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18 15-6-6-6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsDownUp;
impl IconShape for LdChevronsDownUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 20 5-5 5 5" />
  <path d="m7 4 5 5 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsDown;
impl IconShape for LdChevronsDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 6 5 5 5-5" />
  <path d="m7 13 5 5 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsLeftRight;
impl IconShape for LdChevronsLeftRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 7-5 5 5 5" />
  <path d="m15 7 5 5-5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsLeft;
impl IconShape for LdChevronsLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m11 17-5-5 5-5" />
  <path d="m18 17-5-5 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsRightLeft;
impl IconShape for LdChevronsRightLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m20 17-5-5 5-5" />
  <path d="m4 17 5-5-5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsRight;
impl IconShape for LdChevronsRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 17 5-5-5-5" />
  <path d="m13 17 5-5-5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsUpDown;
impl IconShape for LdChevronsUpDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 15 5 5 5-5" />
  <path d="m7 9 5-5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChevronsUp;
impl IconShape for LdChevronsUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 11-5-5-5 5" />
  <path d="m17 18-5-5-5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChrome;
impl IconShape for LdChrome {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <circle cx="12" cy="12" r="4" />
  <line x1="21.17" x2="12" y1="8" y2="8" />
  <line x1="3.95" x2="8.54" y1="6.06" y2="14" />
  <line x1="10.88" x2="15.46" y1="21.94" y2="14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdChurch;
impl IconShape for LdChurch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18 7 4 2v11a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9l4-2" />
  <path d="M14 22v-4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v4" />
  <path d="M18 22V5l-6-3-6 3v17" />
  <path d="M12 7v5" />
  <path d="M10 9h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCigaretteOff;
impl IconShape for LdCigaretteOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="22" y1="2" y2="22" />
  <path d="M12 12H2v4h14" />
  <path d="M22 12v4" />
  <path d="M18 12h-.5" />
  <path d="M7 12v4" />
  <path d="M18 8c0-2.5-2-2.5-2-5" />
  <path d="M22 8c0-2.5-2-2.5-2-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCigarette;
impl IconShape for LdCigarette {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 12H2v4h16" />
  <path d="M22 12v4" />
  <path d="M7 12v4" />
  <path d="M18 8c0-2.5-2-2.5-2-5" />
  <path d="M22 8c0-2.5-2-2.5-2-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleAlert;
impl IconShape for LdCircleAlert {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <line x1="12" x2="12" y1="8" y2="12" />
  <line x1="12" x2="12.01" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowDown;
impl IconShape for LdCircleArrowDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 8v8" />
  <path d="m8 12 4 4 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowLeft;
impl IconShape for LdCircleArrowLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M16 12H8" />
  <path d="m12 8-4 4 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowOutDownLeft;
impl IconShape for LdCircleArrowOutDownLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12a10 10 0 1 1 10 10" />
  <path d="m2 22 10-10" />
  <path d="M8 22H2v-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowOutDownRight;
impl IconShape for LdCircleArrowOutDownRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22a10 10 0 1 1 10-10" />
  <path d="M22 22 12 12" />
  <path d="M22 16v6h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowOutUpLeft;
impl IconShape for LdCircleArrowOutUpLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 8V2h6" />
  <path d="m2 2 10 10" />
  <path d="M12 2A10 10 0 1 1 2 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowOutUpRight;
impl IconShape for LdCircleArrowOutUpRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 12A10 10 0 1 1 12 2" />
  <path d="M22 2 12 12" />
  <path d="M16 2h6v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowRight;
impl IconShape for LdCircleArrowRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M8 12h8" />
  <path d="m12 16 4-4-4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleArrowUp;
impl IconShape for LdCircleArrowUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m16 12-4-4-4 4" />
  <path d="M12 16V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleCheckBig;
impl IconShape for LdCircleCheckBig {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
  <path d="m9 11 3 3L22 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleCheck;
impl IconShape for LdCircleCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m9 12 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleChevronDown;
impl IconShape for LdCircleChevronDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m16 10-4 4-4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleChevronLeft;
impl IconShape for LdCircleChevronLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m14 16-4-4 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleChevronRight;
impl IconShape for LdCircleChevronRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m10 8 4 4-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleChevronUp;
impl IconShape for LdCircleChevronUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m8 14 4-4 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleDashed;
impl IconShape for LdCircleDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.1 2.182a10 10 0 0 1 3.8 0" />
  <path d="M13.9 21.818a10 10 0 0 1-3.8 0" />
  <path d="M17.609 3.721a10 10 0 0 1 2.69 2.7" />
  <path d="M2.182 13.9a10 10 0 0 1 0-3.8" />
  <path d="M20.279 17.609a10 10 0 0 1-2.7 2.69" />
  <path d="M21.818 10.1a10 10 0 0 1 0 3.8" />
  <path d="M3.721 6.391a10 10 0 0 1 2.7-2.69" />
  <path d="M6.391 20.279a10 10 0 0 1-2.69-2.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleDivide;
impl IconShape for LdCircleDivide {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="8" x2="16" y1="12" y2="12" />
  <line x1="12" x2="12" y1="16" y2="16" />
  <line x1="12" x2="12" y1="8" y2="8" />
  <circle cx="12" cy="12" r="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleDollarSign;
impl IconShape for LdCircleDollarSign {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />
  <path d="M12 18V6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleDotDashed;
impl IconShape for LdCircleDotDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0" />
  <path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7" />
  <path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8" />
  <path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69" />
  <path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0" />
  <path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7" />
  <path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8" />
  <path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69" />
  <circle cx="12" cy="12" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleDot;
impl IconShape for LdCircleDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <circle cx="12" cy="12" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleEllipsis;
impl IconShape for LdCircleEllipsis {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M17 12h.01" />
  <path d="M12 12h.01" />
  <path d="M7 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleEqual;
impl IconShape for LdCircleEqual {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 10h10" />
  <path d="M7 14h10" />
  <circle cx="12" cy="12" r="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleFadingPlus;
impl IconShape for LdCircleFadingPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2a10 10 0 0 1 7.38 16.75" />
  <path d="M12 8v8" />
  <path d="M16 12H8" />
  <path d="M2.5 8.875a10 10 0 0 0-.5 3" />
  <path d="M2.83 16a10 10 0 0 0 2.43 3.4" />
  <path d="M4.636 5.235a10 10 0 0 1 .891-.857" />
  <path d="M8.644 21.42a10 10 0 0 0 7.631-.38" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleGauge;
impl IconShape for LdCircleGauge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.6 2.7a10 10 0 1 0 5.7 5.7" />
  <circle cx="12" cy="12" r="2" />
  <path d="M13.4 10.6 19 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleHelp;
impl IconShape for LdCircleHelp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
  <path d="M12 17h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleMinus;
impl IconShape for LdCircleMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M8 12h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleOff;
impl IconShape for LdCircleOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 2 20 20" />
  <path d="M8.35 2.69A10 10 0 0 1 21.3 15.65" />
  <path d="M19.08 19.08A10 10 0 1 1 4.92 4.92" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleParkingOff;
impl IconShape for LdCircleParkingOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m5 5 14 14" />
  <path d="M13 13a3 3 0 1 0 0-6H9v2" />
  <path d="M9 17v-2.34" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleParking;
impl IconShape for LdCircleParking {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M9 17V7h4a3 3 0 0 1 0 6H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCirclePause;
impl IconShape for LdCirclePause {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <line x1="10" x2="10" y1="15" y2="9" />
  <line x1="14" x2="14" y1="15" y2="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCirclePercent;
impl IconShape for LdCirclePercent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m15 9-6 6" />
  <path d="M9 9h.01" />
  <path d="M15 15h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCirclePlay;
impl IconShape for LdCirclePlay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polygon points="10 8 16 12 10 16 10 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCirclePlus;
impl IconShape for LdCirclePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M8 12h8" />
  <path d="M12 8v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCirclePower;
impl IconShape for LdCirclePower {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 12V7" />
  <path d="M16 9a5 5 0 1 1-8 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleSlash2;
impl IconShape for LdCircleSlash2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M22 2 2 22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleSlash;
impl IconShape for LdCircleSlash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="9" x2="15" y1="15" y2="9" />
  <circle cx="12" cy="12" r="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleStop;
impl IconShape for LdCircleStop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <rect width="6" height="6" x="9" y="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleUserRound;
impl IconShape for LdCircleUserRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 20a6 6 0 0 0-12 0" />
  <circle cx="12" cy="10" r="4" />
  <circle cx="12" cy="12" r="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleUser;
impl IconShape for LdCircleUser {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <circle cx="12" cy="10" r="3" />
  <path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircleX;
impl IconShape for LdCircleX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m15 9-6 6" />
  <path d="m9 9 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircle;
impl IconShape for LdCircle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCircuitBoard;
impl IconShape for LdCircuitBoard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M11 9h4a2 2 0 0 0 2-2V3" />
  <circle cx="9" cy="9" r="2" />
  <path d="M7 21v-4a2 2 0 0 1 2-2h4" />
  <circle cx="15" cy="15" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCitrus;
impl IconShape for LdCitrus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.66 17.67a1.08 1.08 0 0 1-.04 1.6A12 12 0 0 1 4.73 2.38a1.1 1.1 0 0 1 1.61-.04z" />
  <path d="M19.65 15.66A8 8 0 0 1 8.35 4.34" />
  <path d="m14 10-5.5 5.5" />
  <path d="M14 17.85V10H6.15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClapperboard;
impl IconShape for LdClapperboard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3Z" />
  <path d="m6.2 5.3 3.1 3.9" />
  <path d="m12.4 3.4 3.1 4" />
  <path d="M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardCheck;
impl IconShape for LdClipboardCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
  <path d="m9 14 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardCopy;
impl IconShape for LdClipboardCopy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2" />
  <path d="M16 4h2a2 2 0 0 1 2 2v4" />
  <path d="M21 14H11" />
  <path d="m15 10-4 4 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardList;
impl IconShape for LdClipboardList {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
  <path d="M12 11h4" />
  <path d="M12 16h4" />
  <path d="M8 11h.01" />
  <path d="M8 16h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardMinus;
impl IconShape for LdClipboardMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
  <path d="M9 14h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardPaste;
impl IconShape for LdClipboardPaste {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H9a1 1 0 0 0-1 1v2c0 .6.4 1 1 1h6c.6 0 1-.4 1-1V3c0-.6-.4-1-1-1Z" />
  <path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2M16 4h2a2 2 0 0 1 2 2v2M11 14h10" />
  <path d="m17 10 4 4-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardPenLine;
impl IconShape for LdClipboardPenLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" />
  <path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5" />
  <path d="M16 4h2a2 2 0 0 1 1.73 1" />
  <path d="M8 18h1" />
  <path d="M18.4 9.6a2 2 0 0 1 3 3L17 17l-4 1 1-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardPen;
impl IconShape for LdClipboardPen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" />
  <path d="M10.4 12.6a2 2 0 0 1 3 3L8 21l-4 1 1-4Z" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-5.5" />
  <path d="M4 13.5V6a2 2 0 0 1 2-2h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardPlus;
impl IconShape for LdClipboardPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
  <path d="M9 14h6" />
  <path d="M12 17v-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardType;
impl IconShape for LdClipboardType {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
  <path d="M9 12v-1h6v1" />
  <path d="M11 17h2" />
  <path d="M12 11v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboardX;
impl IconShape for LdClipboardX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
  <path d="m15 11-6 6" />
  <path d="m9 11 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClipboard;
impl IconShape for LdClipboard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="4" x="8" y="2" rx="1" ry="1" />
  <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock1;
impl IconShape for LdClock1 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 14.5 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock10;
impl IconShape for LdClock10 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 8 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock11;
impl IconShape for LdClock11 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 9.5 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock12;
impl IconShape for LdClock12 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock2;
impl IconShape for LdClock2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 16 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock3;
impl IconShape for LdClock3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 16.5 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock4;
impl IconShape for LdClock4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 16 14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock5;
impl IconShape for LdClock5 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 14.5 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock6;
impl IconShape for LdClock6 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 12 16.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock7;
impl IconShape for LdClock7 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 9.5 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock8;
impl IconShape for LdClock8 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 8 14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock9;
impl IconShape for LdClock9 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 7.5 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClock;
impl IconShape for LdClock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polyline points="12 6 12 12 16 14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudCog;
impl IconShape for LdCloudCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="17" r="3" />
  <path d="M4.2 15.1A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2" />
  <path d="m15.7 18.4-.9-.3" />
  <path d="m9.2 15.9-.9-.3" />
  <path d="m10.6 20.7.3-.9" />
  <path d="m13.1 14.2.3-.9" />
  <path d="m13.6 20.7-.4-1" />
  <path d="m10.8 14.3-.4-1" />
  <path d="m8.3 18.6 1-.4" />
  <path d="m14.7 15.8 1-.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudDownload;
impl IconShape for LdCloudDownload {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M12 12v9" />
  <path d="m8 17 4 4 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudDrizzle;
impl IconShape for LdCloudDrizzle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M8 19v1" />
  <path d="M8 14v1" />
  <path d="M16 19v1" />
  <path d="M16 14v1" />
  <path d="M12 21v1" />
  <path d="M12 16v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudFog;
impl IconShape for LdCloudFog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M16 17H7" />
  <path d="M17 21H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudHail;
impl IconShape for LdCloudHail {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M16 14v2" />
  <path d="M8 14v2" />
  <path d="M16 20h.01" />
  <path d="M8 20h.01" />
  <path d="M12 16v2" />
  <path d="M12 22h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudLightning;
impl IconShape for LdCloudLightning {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973" />
  <path d="m13 12-3 5h4l-3 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudMoonRain;
impl IconShape for LdCloudMoonRain {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.083 9A6.002 6.002 0 0 1 16 4a4.243 4.243 0 0 0 6 6c0 2.22-1.206 4.16-3 5.197" />
  <path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" />
  <path d="M11 20v2" />
  <path d="M7 19v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudMoon;
impl IconShape for LdCloudMoon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 16a3 3 0 1 1 0 6H7a5 5 0 1 1 4.9-6Z" />
  <path d="M10.1 9A6 6 0 0 1 16 4a4.24 4.24 0 0 0 6 6 6 6 0 0 1-3 5.197" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudOff;
impl IconShape for LdCloudOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 2 20 20" />
  <path d="M5.782 5.782A7 7 0 0 0 9 19h8.5a4.5 4.5 0 0 0 1.307-.193" />
  <path d="M21.532 16.5A4.5 4.5 0 0 0 17.5 10h-1.79A7.008 7.008 0 0 0 10 5.07" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudRainWind;
impl IconShape for LdCloudRainWind {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="m9.2 22 3-7" />
  <path d="m9 13-3 7" />
  <path d="m17 13-3 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudRain;
impl IconShape for LdCloudRain {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M16 14v6" />
  <path d="M8 14v6" />
  <path d="M12 16v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudSnow;
impl IconShape for LdCloudSnow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M8 15h.01" />
  <path d="M8 19h.01" />
  <path d="M12 17h.01" />
  <path d="M12 21h.01" />
  <path d="M16 15h.01" />
  <path d="M16 19h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudSunRain;
impl IconShape for LdCloudSunRain {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v2" />
  <path d="m4.93 4.93 1.41 1.41" />
  <path d="M20 12h2" />
  <path d="m19.07 4.93-1.41 1.41" />
  <path d="M15.947 12.65a4 4 0 0 0-5.925-4.128" />
  <path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" />
  <path d="M11 20v2" />
  <path d="M7 19v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudSun;
impl IconShape for LdCloudSun {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v2" />
  <path d="m4.93 4.93 1.41 1.41" />
  <path d="M20 12h2" />
  <path d="m19.07 4.93-1.41 1.41" />
  <path d="M15.947 12.65a4 4 0 0 0-5.925-4.128" />
  <path d="M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudUpload;
impl IconShape for LdCloudUpload {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
  <path d="M12 12v9" />
  <path d="m16 16-4-4-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloud;
impl IconShape for LdCloud {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCloudy;
impl IconShape for LdCloudy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.5 21H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z" />
  <path d="M22 10a3 3 0 0 0-3-3h-2.207a5.502 5.502 0 0 0-10.702.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClover;
impl IconShape for LdClover {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16.17 7.83 2 22" />
  <path d="M4.02 12a2.827 2.827 0 1 1 3.81-4.17A2.827 2.827 0 1 1 12 4.02a2.827 2.827 0 1 1 4.17 3.81A2.827 2.827 0 1 1 19.98 12a2.827 2.827 0 1 1-3.81 4.17A2.827 2.827 0 1 1 12 19.98a2.827 2.827 0 1 1-4.17-3.81A1 1 0 1 1 4 12" />
  <path d="m7.83 7.83 8.34 8.34" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdClub;
impl IconShape for LdClub {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z" />
  <path d="M12 17.66L12 22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCodeXml;
impl IconShape for LdCodeXml {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18 16 4-4-4-4" />
  <path d="m6 8-4 4 4 4" />
  <path d="m14.5 4-5 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCode;
impl IconShape for LdCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="16 18 22 12 16 6" />
  <polyline points="8 6 2 12 8 18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCodepen;
impl IconShape for LdCodepen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" />
  <line x1="12" x2="12" y1="22" y2="15.5" />
  <polyline points="22 8.5 12 15.5 2 8.5" />
  <polyline points="2 15.5 12 8.5 22 15.5" />
  <line x1="12" x2="12" y1="2" y2="8.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCodesandbox;
impl IconShape for LdCodesandbox {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
  <polyline points="7.5 4.21 12 6.81 16.5 4.21" />
  <polyline points="7.5 19.79 7.5 14.6 3 12" />
  <polyline points="21 12 16.5 14.6 16.5 19.79" />
  <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
  <line x1="12" x2="12" y1="22.08" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCoffee;
impl IconShape for LdCoffee {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 2v2" />
  <path d="M14 2v2" />
  <path d="M16 8a1 1 0 0 1 1 1v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1h14a4 4 0 1 1 0 8h-1" />
  <path d="M6 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCog;
impl IconShape for LdCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" />
  <path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" />
  <path d="M12 2v2" />
  <path d="M12 22v-2" />
  <path d="m17 20.66-1-1.73" />
  <path d="M11 10.27 7 3.34" />
  <path d="m20.66 17-1.73-1" />
  <path d="m3.34 7 1.73 1" />
  <path d="M14 12h8" />
  <path d="M2 12h2" />
  <path d="m20.66 7-1.73 1" />
  <path d="m3.34 17 1.73-1" />
  <path d="m17 3.34-1 1.73" />
  <path d="m11 13.73-4 6.93" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCoins;
impl IconShape for LdCoins {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="8" cy="8" r="6" />
  <path d="M18.09 10.37A6 6 0 1 1 10.34 18" />
  <path d="M7 6h1v4" />
  <path d="m16.71 13.88.7.71-2.82 2.82" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdColumns2;
impl IconShape for LdColumns2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M12 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdColumns3;
impl IconShape for LdColumns3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 3v18" />
  <path d="M15 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdColumns4;
impl IconShape for LdColumns4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7.5 3v18" />
  <path d="M12 3v18" />
  <path d="M16.5 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCombine;
impl IconShape for LdCombine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="8" x="2" y="2" rx="2" />
  <path d="M14 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
  <path d="M20 2c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
  <path d="M10 18H5c-1.7 0-3-1.3-3-3v-1" />
  <polyline points="7 21 10 18 7 15" />
  <rect width="8" height="8" x="14" y="14" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCommand;
impl IconShape for LdCommand {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCompass;
impl IconShape for LdCompass {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdComponent;
impl IconShape for LdComponent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5.5 8.5 9 12l-3.5 3.5L2 12l3.5-3.5Z" />
  <path d="m12 2 3.5 3.5L12 9 8.5 5.5 12 2Z" />
  <path d="M18.5 8.5 22 12l-3.5 3.5L15 12l3.5-3.5Z" />
  <path d="m12 15 3.5 3.5L12 22l-3.5-3.5L12 15Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdComputer;
impl IconShape for LdComputer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="8" x="5" y="2" rx="2" />
  <rect width="20" height="8" x="2" y="14" rx="2" />
  <path d="M6 18h2" />
  <path d="M12 18h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdConciergeBell;
impl IconShape for LdConciergeBell {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 20a1 1 0 0 1-1-1v-1a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1Z" />
  <path d="M20 16a8 8 0 1 0-16 0" />
  <path d="M12 4v4" />
  <path d="M10 4h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCone;
impl IconShape for LdCone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98" />
  <ellipse cx="12" cy="19" rx="9" ry="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdConstruction;
impl IconShape for LdConstruction {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="2" y="6" width="20" height="8" rx="1" />
  <path d="M17 14v7" />
  <path d="M7 14v7" />
  <path d="M17 3v3" />
  <path d="M7 3v3" />
  <path d="M10 14 2.3 6.3" />
  <path d="m14 6 7.7 7.7" />
  <path d="m8 6 8 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdContactRound;
impl IconShape for LdContactRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 18a4 4 0 0 0-8 0" />
  <circle cx="12" cy="11" r="3" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <line x1="8" x2="8" y1="2" y2="4" />
  <line x1="16" x2="16" y1="2" y2="4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdContact;
impl IconShape for LdContact {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" />
  <rect width="18" height="18" x="3" y="4" rx="2" />
  <circle cx="12" cy="10" r="2" />
  <line x1="8" x2="8" y1="2" y2="4" />
  <line x1="16" x2="16" y1="2" y2="4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdContainer;
impl IconShape for LdContainer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z" />
  <path d="M10 21.9V14L2.1 9.1" />
  <path d="m10 14 11.9-6.9" />
  <path d="M14 19.8v-8.1" />
  <path d="M18 17.5V9.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdContrast;
impl IconShape for LdContrast {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 18a6 6 0 0 0 0-12v12z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCookie;
impl IconShape for LdCookie {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5" />
  <path d="M8.5 8.5v.01" />
  <path d="M16 15.5v.01" />
  <path d="M12 12v.01" />
  <path d="M11 17v.01" />
  <path d="M7 14v.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCookingPot;
impl IconShape for LdCookingPot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12h20" />
  <path d="M20 12v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-8" />
  <path d="m4 8 16-4" />
  <path d="m8.86 6.78-.45-1.81a2 2 0 0 1 1.45-2.43l1.94-.48a2 2 0 0 1 2.43 1.46l.45 1.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopyCheck;
impl IconShape for LdCopyCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 15 2 2 4-4" />
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopyMinus;
impl IconShape for LdCopyMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="18" y1="15" y2="15" />
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopyPlus;
impl IconShape for LdCopyPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="15" x2="15" y1="12" y2="18" />
  <line x1="12" x2="18" y1="15" y2="15" />
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopySlash;
impl IconShape for LdCopySlash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="18" y1="18" y2="12" />
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopyX;
impl IconShape for LdCopyX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="18" y1="12" y2="18" />
  <line x1="12" x2="18" y1="18" y2="12" />
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopy;
impl IconShape for LdCopy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2" />
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopyleft;
impl IconShape for LdCopyleft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M9.17 14.83a4 4 0 1 0 0-5.66" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCopyright;
impl IconShape for LdCopyright {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M14.83 14.83a4 4 0 1 1 0-5.66" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerDownLeft;
impl IconShape for LdCornerDownLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="9 10 4 15 9 20" />
  <path d="M20 4v7a4 4 0 0 1-4 4H4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerDownRight;
impl IconShape for LdCornerDownRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="15 10 20 15 15 20" />
  <path d="M4 4v7a4 4 0 0 0 4 4h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerLeftDown;
impl IconShape for LdCornerLeftDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="14 15 9 20 4 15" />
  <path d="M20 4h-7a4 4 0 0 0-4 4v12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerLeftUp;
impl IconShape for LdCornerLeftUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="14 9 9 4 4 9" />
  <path d="M20 20h-7a4 4 0 0 1-4-4V4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerRightDown;
impl IconShape for LdCornerRightDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="10 15 15 20 20 15" />
  <path d="M4 4h7a4 4 0 0 1 4 4v12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerRightUp;
impl IconShape for LdCornerRightUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="10 9 15 4 20 9" />
  <path d="M4 20h7a4 4 0 0 0 4-4V4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerUpLeft;
impl IconShape for LdCornerUpLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="9 14 4 9 9 4" />
  <path d="M20 20v-7a4 4 0 0 0-4-4H4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCornerUpRight;
impl IconShape for LdCornerUpRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="15 14 20 9 15 4" />
  <path d="M4 20v-7a4 4 0 0 1 4-4h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCpu;
impl IconShape for LdCpu {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect   x="4" y="4" rx="2" />
  <rect width="6" height="6" x="9" y="9" rx="1" />
  <path d="M15 2v2" />
  <path d="M15 20v2" />
  <path d="M2 15h2" />
  <path d="M2 9h2" />
  <path d="M20 15h2" />
  <path d="M20 9h2" />
  <path d="M9 2v2" />
  <path d="M9 20v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCreativeCommons;
impl IconShape for LdCreativeCommons {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M10 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1" />
  <path d="M17 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCreditCard;
impl IconShape for LdCreditCard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="14" x="2" y="5" rx="2" />
  <line x1="2" x2="22" y1="10" y2="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCroissant;
impl IconShape for LdCroissant {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m4.6 13.11 5.79-3.21c1.89-1.05 4.79 1.78 3.71 3.71l-3.22 5.81C8.8 23.16.79 15.23 4.6 13.11Z" />
  <path d="m10.5 9.5-1-2.29C9.2 6.48 8.8 6 8 6H4.5C2.79 6 2 6.5 2 8.5a7.71 7.71 0 0 0 2 4.83" />
  <path d="M8 6c0-1.55.24-4-2-4-2 0-2.5 2.17-2.5 4" />
  <path d="m14.5 13.5 2.29 1c.73.3 1.21.7 1.21 1.5v3.5c0 1.71-.5 2.5-2.5 2.5a7.71 7.71 0 0 1-4.83-2" />
  <path d="M18 16c1.55 0 4-.24 4 2 0 2-2.17 2.5-4 2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCrop;
impl IconShape for LdCrop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 2v14a2 2 0 0 0 2 2h14" />
  <path d="M18 22V8a2 2 0 0 0-2-2H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCross;
impl IconShape for LdCross {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 2a2 2 0 0 0-2 2v5H4a2 2 0 0 0-2 2v2c0 1.1.9 2 2 2h5v5c0 1.1.9 2 2 2h2a2 2 0 0 0 2-2v-5h5a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-5V4a2 2 0 0 0-2-2h-2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCrosshair;
impl IconShape for LdCrosshair {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <line x1="22" x2="18" y1="12" y2="12" />
  <line x1="6" x2="2" y1="12" y2="12" />
  <line x1="12" x2="12" y1="6" y2="2" />
  <line x1="12" x2="12" y1="22" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCrown;
impl IconShape for LdCrown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11.562 3.266a.5.5 0 0 1 .876 0L15.39 8.87a1 1 0 0 0 1.516.294L21.183 5.5a.5.5 0 0 1 .798.519l-2.834 10.246a1 1 0 0 1-.956.734H5.81a1 1 0 0 1-.957-.734L2.02 6.02a.5.5 0 0 1 .798-.519l4.276 3.664a1 1 0 0 0 1.516-.294z" />
  <path d="M5 21h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCuboid;
impl IconShape for LdCuboid {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21.12 6.4-6.05-4.06a2 2 0 0 0-2.17-.05L2.95 8.41a2 2 0 0 0-.95 1.7v5.82a2 2 0 0 0 .88 1.66l6.05 4.07a2 2 0 0 0 2.17.05l9.95-6.12a2 2 0 0 0 .95-1.7V8.06a2 2 0 0 0-.88-1.66Z" />
  <path d="M10 22v-8L2.25 9.15" />
  <path d="m10 14 11.77-6.87" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCupSoda;
impl IconShape for LdCupSoda {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8" />
  <path d="M5 8h14" />
  <path d="M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0" />
  <path d="m12 8 1-6h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCurrency;
impl IconShape for LdCurrency {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="8" />
  <line x1="3" x2="6" y1="3" y2="6" />
  <line x1="21" x2="18" y1="3" y2="6" />
  <line x1="3" x2="6" y1="21" y2="18" />
  <line x1="21" x2="18" y1="21" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdCylinder;
impl IconShape for LdCylinder {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <ellipse cx="12" cy="5" rx="9" ry="3" />
  <path d="M3 5v14a9 3 0 0 0 18 0V5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDatabaseBackup;
impl IconShape for LdDatabaseBackup {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <ellipse cx="12" cy="5" rx="9" ry="3" />
  <path d="M3 12a9 3 0 0 0 5 2.69" />
  <path d="M21 9.3V5" />
  <path d="M3 5v14a9 3 0 0 0 6.47 2.88" />
  <path d="M12 12v4h4" />
  <path d="M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDatabaseZap;
impl IconShape for LdDatabaseZap {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <ellipse cx="12" cy="5" rx="9" ry="3" />
  <path d="M3 5V19A9 3 0 0 0 15 21.84" />
  <path d="M21 5V8" />
  <path d="M21 12L18 17H22L19 22" />
  <path d="M3 12A9 3 0 0 0 14.59 14.87" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDatabase;
impl IconShape for LdDatabase {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <ellipse cx="12" cy="5" rx="9" ry="3" />
  <path d="M3 5V19A9 3 0 0 0 21 19V5" />
  <path d="M3 12A9 3 0 0 0 21 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDelete;
impl IconShape for LdDelete {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 5H9l-7 7 7 7h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2Z" />
  <line x1="18" x2="12" y1="9" y2="15" />
  <line x1="12" x2="18" y1="9" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDessert;
impl IconShape for LdDessert {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="4" r="2" />
  <path d="M10.2 3.2C5.5 4 2 8.1 2 13a2 2 0 0 0 4 0v-1a2 2 0 0 1 4 0v4a2 2 0 0 0 4 0v-4a2 2 0 0 1 4 0v1a2 2 0 0 0 4 0c0-4.9-3.5-9-8.2-9.8" />
  <path d="M3.2 14.8a9 9 0 0 0 17.6 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiameter;
impl IconShape for LdDiameter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="19" cy="19" r="2" />
  <circle cx="5" cy="5" r="2" />
  <path d="M6.48 3.66a10 10 0 0 1 13.86 13.86" />
  <path d="m6.41 6.41 11.18 11.18" />
  <path d="M3.66 6.48a10 10 0 0 0 13.86 13.86" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiamondMinus;
impl IconShape for LdDiamondMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0z" />
  <path d="M8 12h8" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiamondPercent;
impl IconShape for LdDiamondPercent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0Z" />
  <path d="M9.2 9.2h.01" />
  <path d="m14.5 9.5-5 5" />
  <path d="M14.7 14.8h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiamondPlus;
impl IconShape for LdDiamondPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 8v8" />
  <path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0z" />
  <path d="M8 12h8" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiamond;
impl IconShape for LdDiamond {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDice1;
impl IconShape for LdDice1 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M12 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDice2;
impl IconShape for LdDice2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M15 9h.01" />
  <path d="M9 15h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDice3;
impl IconShape for LdDice3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M16 8h.01" />
  <path d="M12 12h.01" />
  <path d="M8 16h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDice4;
impl IconShape for LdDice4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M16 8h.01" />
  <path d="M8 8h.01" />
  <path d="M8 16h.01" />
  <path d="M16 16h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDice5;
impl IconShape for LdDice5 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M16 8h.01" />
  <path d="M8 8h.01" />
  <path d="M8 16h.01" />
  <path d="M16 16h.01" />
  <path d="M12 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDice6;
impl IconShape for LdDice6 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M16 8h.01" />
  <path d="M16 12h.01" />
  <path d="M16 16h.01" />
  <path d="M8 8h.01" />
  <path d="M8 12h.01" />
  <path d="M8 16h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDices;
impl IconShape for LdDices {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="12" height="12" x="2" y="10" rx="2" ry="2" />
  <path d="m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6" />
  <path d="M6 18h.01" />
  <path d="M10 14h.01" />
  <path d="M15 6h.01" />
  <path d="M18 9h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiff;
impl IconShape for LdDiff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3v14" />
  <path d="M5 10h14" />
  <path d="M5 21h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDisc2;
impl IconShape for LdDisc2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <circle cx="12" cy="12" r="4" />
  <path d="M12 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDisc3;
impl IconShape for LdDisc3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M6 12c0-1.7.7-3.2 1.8-4.2" />
  <circle cx="12" cy="12" r="2" />
  <path d="M18 12c0 1.7-.7 3.2-1.8 4.2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDiscAlbum;
impl IconShape for LdDiscAlbum {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <circle cx="12" cy="12" r="5" />
  <path d="M12 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDisc;
impl IconShape for LdDisc {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <circle cx="12" cy="12" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDivide;
impl IconShape for LdDivide {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="6" r="1" />
  <line x1="5" x2="19" y1="12" y2="12" />
  <circle cx="12" cy="18" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDnaOff;
impl IconShape for LdDnaOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2c-1.35 1.5-2.092 3-2.5 4.5M9 22c1.35-1.5 2.092-3 2.5-4.5" />
  <path d="M2 15c3.333-3 6.667-3 10-3m10-3c-1.5 1.35-3 2.092-4.5 2.5" />
  <path d="m17 6-2.5-2.5" />
  <path d="m14 8-1.5-1.5" />
  <path d="m7 18 2.5 2.5" />
  <path d="m3.5 14.5.5.5" />
  <path d="m20 9 .5.5" />
  <path d="m6.5 12.5 1 1" />
  <path d="m16.5 10.5 1 1" />
  <path d="m10 16 1.5 1.5" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDna;
impl IconShape for LdDna {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 15c6.667-6 13.333 0 20-6" />
  <path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993" />
  <path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993" />
  <path d="m17 6-2.5-2.5" />
  <path d="m14 8-1-1" />
  <path d="m7 18 2.5 2.5" />
  <path d="m3.5 14.5.5.5" />
  <path d="m20 9 .5.5" />
  <path d="m6.5 12.5 1 1" />
  <path d="m16.5 10.5 1 1" />
  <path d="m10 16 1.5 1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDock;
impl IconShape for LdDock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 8h20" />
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="M6 16h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDog;
impl IconShape for LdDog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 5.172C10 3.782 8.423 2.679 6.5 3c-2.823.47-4.113 6.006-4 7 .08.703 1.725 1.722 3.656 1 1.261-.472 1.96-1.45 2.344-2.5" />
  <path d="M14.267 5.172c0-1.39 1.577-2.493 3.5-2.172 2.823.47 4.113 6.006 4 7-.08.703-1.725 1.722-3.656 1-1.261-.472-1.855-1.45-2.239-2.5" />
  <path d="M8 14v.5" />
  <path d="M16 14v.5" />
  <path d="M11.25 16.25h1.5L12 17l-.75-.75Z" />
  <path d="M4.42 11.247A13.152 13.152 0 0 0 4 14.556C4 18.728 7.582 21 12 21s8-2.272 8-6.444c0-1.061-.162-2.2-.493-3.309m-9.243-6.082A8.801 8.801 0 0 1 12 5c.78 0 1.5.108 2.161.306" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDollarSign;
impl IconShape for LdDollarSign {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="12" y1="2" y2="22" />
  <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDonut;
impl IconShape for LdDonut {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20.5 10a2.5 2.5 0 0 1-2.4-3H18a2.95 2.95 0 0 1-2.6-4.4 10 10 0 1 0 6.3 7.1c-.3.2-.8.3-1.2.3" />
  <circle cx="12" cy="12" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDoorClosed;
impl IconShape for LdDoorClosed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 20V6a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v14" />
  <path d="M2 20h20" />
  <path d="M14 12v.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDoorOpen;
impl IconShape for LdDoorOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 4h3a2 2 0 0 1 2 2v14" />
  <path d="M2 20h3" />
  <path d="M13 20h9" />
  <path d="M10 12v.01" />
  <path d="M13 4.562v16.157a1 1 0 0 1-1.242.97L5 20V5.562a2 2 0 0 1 1.515-1.94l4-1A2 2 0 0 1 13 4.561Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDot;
impl IconShape for LdDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12.1" cy="12.1" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDownload;
impl IconShape for LdDownload {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
  <polyline points="7 10 12 15 17 10" />
  <line x1="12" x2="12" y1="15" y2="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDraftingCompass;
impl IconShape for LdDraftingCompass {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="5" r="2" />
  <path d="m3 21 8.02-14.26" />
  <path d="m12.99 6.74 1.93 3.44" />
  <path d="M19 12c-3.87 4-10.13 4-14 0" />
  <path d="m21 21-2.16-3.84" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDrama;
impl IconShape for LdDrama {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 11h.01" />
  <path d="M14 6h.01" />
  <path d="M18 6h.01" />
  <path d="M6.5 13.1h.01" />
  <path d="M22 5c0 9-4 12-6 12s-6-3-6-12c0-2 2-3 6-3s6 1 6 3" />
  <path d="M17.4 9.9c-.8.8-2 .8-2.8 0" />
  <path d="M10.1 7.1C9 7.2 7.7 7.7 6 8.6c-3.5 2-4.7 3.9-3.7 5.6 4.5 7.8 9.5 8.4 11.2 7.4.9-.5 1.9-2.1 1.9-4.7" />
  <path d="M9.1 16.5c.3-1.1 1.4-1.7 2.4-1.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDribbble;
impl IconShape for LdDribbble {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M19.13 5.09C15.22 9.14 10 10.44 2.25 10.94" />
  <path d="M21.75 12.84c-6.62-1.41-12.14 1-16.38 6.32" />
  <path d="M8.56 2.75c4.37 6 6 9.42 8 17.72" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDrill;
impl IconShape for LdDrill {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 9c0 .6-.4 1-1 1H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9c.6 0 1 .4 1 1Z" />
  <path d="M18 6h4" />
  <path d="M14 4h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3" />
  <path d="m5 10-2 8" />
  <path d="M12 10v3c0 .6-.4 1-1 1H8" />
  <path d="m7 18 2-8" />
  <path d="M5 22c-1.7 0-3-1.3-3-3 0-.6.4-1 1-1h7c.6 0 1 .4 1 1v2c0 .6-.4 1-1 1Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDroplet;
impl IconShape for LdDroplet {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDroplets;
impl IconShape for LdDroplets {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 16.3c2.2 0 4-1.83 4-4.05 0-1.16-.57-2.26-1.71-3.19S7.29 6.75 7 5.3c-.29 1.45-1.14 2.84-2.29 3.76S3 11.1 3 12.25c0 2.22 1.8 4.05 4 4.05z" />
  <path d="M12.56 6.6A10.97 10.97 0 0 0 14 3.02c.5 2.5 2 4.9 4 6.5s3 3.5 3 5.5a6.98 6.98 0 0 1-11.91 4.97" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDrum;
impl IconShape for LdDrum {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 2 8 8" />
  <path d="m22 2-8 8" />
  <ellipse cx="12" cy="9" rx="10" ry="5" />
  <path d="M7 13.4v7.9" />
  <path d="M12 14v8" />
  <path d="M17 13.4v7.9" />
  <path d="M2 9v8a10 5 0 0 0 20 0V9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDrumstick;
impl IconShape for LdDrumstick {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.4 15.63a7.875 6 135 1 1 6.23-6.23 4.5 3.43 135 0 0-6.23 6.23" />
  <path d="m8.29 12.71-2.6 2.6a2.5 2.5 0 1 0-1.65 4.65A2.5 2.5 0 1 0 8.7 18.3l2.59-2.59" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdDumbbell;
impl IconShape for LdDumbbell {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.4 14.4 9.6 9.6" />
  <path d="M18.657 21.485a2 2 0 1 1-2.829-2.828l-1.767 1.768a2 2 0 1 1-2.829-2.829l6.364-6.364a2 2 0 1 1 2.829 2.829l-1.768 1.767a2 2 0 1 1 2.828 2.829z" />
  <path d="m21.5 21.5-1.4-1.4" />
  <path d="M3.9 3.9 2.5 2.5" />
  <path d="M6.404 12.768a2 2 0 1 1-2.829-2.829l1.768-1.767a2 2 0 1 1-2.828-2.829l2.828-2.828a2 2 0 1 1 2.829 2.828l1.767-1.768a2 2 0 1 1 2.829 2.829z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEarOff;
impl IconShape for LdEarOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 18.5a3.5 3.5 0 1 0 7 0c0-1.57.92-2.52 2.04-3.46" />
  <path d="M6 8.5c0-.75.13-1.47.36-2.14" />
  <path d="M8.8 3.15A6.5 6.5 0 0 1 19 8.5c0 1.63-.44 2.81-1.09 3.76" />
  <path d="M12.5 6A2.5 2.5 0 0 1 15 8.5M10 13a2 2 0 0 0 1.82-1.18" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEar;
impl IconShape for LdEar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0" />
  <path d="M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEarthLock;
impl IconShape for LdEarthLock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 3.34V5a3 3 0 0 0 3 3" />
  <path d="M11 21.95V18a2 2 0 0 0-2-2 2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05" />
  <path d="M21.54 15H17a2 2 0 0 0-2 2v4.54" />
  <path d="M12 2a10 10 0 1 0 9.54 13" />
  <path d="M20 6V4a2 2 0 1 0-4 0v2" />
  <rect width="8" height="5" x="14" y="6" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEarth;
impl IconShape for LdEarth {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.54 15H17a2 2 0 0 0-2 2v4.54" />
  <path d="M7 3.34V5a3 3 0 0 0 3 3v0a2 2 0 0 1 2 2v0c0 1.1.9 2 2 2v0a2 2 0 0 0 2-2v0c0-1.1.9-2 2-2h3.17" />
  <path d="M11 21.95V18a2 2 0 0 0-2-2v0a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05" />
  <circle cx="12" cy="12" r="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEclipse;
impl IconShape for LdEclipse {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 2a7 7 0 1 0 10 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEggFried;
impl IconShape for LdEggFried {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11.5" cy="12.5" r="3.5" />
  <path d="M3 8c0-3.5 2.5-6 6.5-6 5 0 4.83 3 7.5 5s5 2 5 6c0 4.5-2.5 6.5-7 6.5-2.5 0-2.5 2.5-6 2.5s-7-2-7-5.5c0-3 1.5-3 1.5-5C3.5 10 3 9 3 8Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEggOff;
impl IconShape for LdEggOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6.399 6.399C5.362 8.157 4.65 10.189 4.5 12c-.37 4.43 1.27 9.95 7.5 10 3.256-.026 5.259-1.547 6.375-3.625" />
  <path d="M19.532 13.875A14.07 14.07 0 0 0 19.5 12c-.36-4.34-3.95-9.96-7.5-10-1.04.012-2.082.502-3.046 1.297" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEgg;
impl IconShape for LdEgg {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22c6.23-.05 7.87-5.57 7.5-10-.36-4.34-3.95-9.96-7.5-10-3.55.04-7.14 5.66-7.5 10-.37 4.43 1.27 9.95 7.5 10z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEllipsisVertical;
impl IconShape for LdEllipsisVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="1" />
  <circle cx="12" cy="5" r="1" />
  <circle cx="12" cy="19" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEllipsis;
impl IconShape for LdEllipsis {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="1" />
  <circle cx="19" cy="12" r="1" />
  <circle cx="5" cy="12" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEqualNot;
impl IconShape for LdEqualNot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="5" x2="19" y1="9" y2="9" />
  <line x1="5" x2="19" y1="15" y2="15" />
  <line x1="19" x2="5" y1="5" y2="19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEqual;
impl IconShape for LdEqual {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="5" x2="19" y1="9" y2="9" />
  <line x1="5" x2="19" y1="15" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEraser;
impl IconShape for LdEraser {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 21-4.3-4.3c-1-1-1-2.5 0-3.4l9.6-9.6c1-1 2.5-1 3.4 0l5.6 5.6c1 1 1 2.5 0 3.4L13 21" />
  <path d="M22 21H7" />
  <path d="m5 11 9 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEuro;
impl IconShape for LdEuro {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 10h12" />
  <path d="M4 14h9" />
  <path d="M19 6a7.7 7.7 0 0 0-5.2-2A7.9 7.9 0 0 0 6 12c0 4.4 3.5 8 7.8 8 2 0 3.8-.8 5.2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdExpand;
impl IconShape for LdExpand {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21 21-6-6m6 6v-4.8m0 4.8h-4.8" />
  <path d="M3 16.2V21m0 0h4.8M3 21l6-6" />
  <path d="M21 7.8V3m0 0h-4.8M21 3l-6 6" />
  <path d="M3 7.8V3m0 0h4.8M3 3l6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdExternalLink;
impl IconShape for LdExternalLink {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 3h6v6" />
  <path d="M10 14 21 3" />
  <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEyeOff;
impl IconShape for LdEyeOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
  <path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68" />
  <path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdEye;
impl IconShape for LdEye {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" />
  <circle cx="12" cy="12" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFacebook;
impl IconShape for LdFacebook {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFactory;
impl IconShape for LdFactory {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8l-7 5V8l-7 5V4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="M17 18h1" />
  <path d="M12 18h1" />
  <path d="M7 18h1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFan;
impl IconShape for LdFan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.827 16.379a6.082 6.082 0 0 1-8.618-7.002l5.412 1.45a6.082 6.082 0 0 1 7.002-8.618l-1.45 5.412a6.082 6.082 0 0 1 8.618 7.002l-5.412-1.45a6.082 6.082 0 0 1-7.002 8.618l1.45-5.412Z" />
  <path d="M12 12v.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFastForward;
impl IconShape for LdFastForward {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="13 19 22 12 13 5 13 19" />
  <polygon points="2 19 11 12 2 5 2 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFeather;
impl IconShape for LdFeather {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.67 19a2 2 0 0 0 1.416-.588l6.154-6.172a6 6 0 0 0-8.49-8.49L5.586 9.914A2 2 0 0 0 5 11.328V18a1 1 0 0 0 1 1z" />
  <path d="M16 8 2 22" />
  <path d="M17.5 15H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFence;
impl IconShape for LdFence {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 3 2 5v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" />
  <path d="M6 8h4" />
  <path d="M6 18h4" />
  <path d="m12 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" />
  <path d="M14 8h4" />
  <path d="M14 18h4" />
  <path d="m20 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFerrisWheel;
impl IconShape for LdFerrisWheel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="2" />
  <path d="M12 2v4" />
  <path d="m6.8 15-3.5 2" />
  <path d="m20.7 7-3.5 2" />
  <path d="M6.8 9 3.3 7" />
  <path d="m20.7 17-3.5-2" />
  <path d="m9 22 3-8 3 8" />
  <path d="M8 22h8" />
  <path d="M18 18.7a9 9 0 1 0-12 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFigma;
impl IconShape for LdFigma {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z" />
  <path d="M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z" />
  <path d="M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z" />
  <path d="M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z" />
  <path d="M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileArchive;
impl IconShape for LdFileArchive {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v18" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="10" cy="20" r="2" />
  <path d="M10 7V6" />
  <path d="M10 12v-1" />
  <path d="M10 18v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileAudio2;
impl IconShape for LdFileAudio2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="3" cy="17" r="1" />
  <path d="M2 17v-3a4 4 0 0 1 8 0v3" />
  <circle cx="9" cy="17" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileAudio;
impl IconShape for LdFileAudio {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.5 22h.5a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M2 19a2 2 0 1 1 4 0v1a2 2 0 1 1-4 0v-4a6 6 0 0 1 12 0v4a2 2 0 1 1-4 0v-1a2 2 0 1 1 4 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileAxis3d;
impl IconShape for LdFileAxis3d {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m8 18 4-4" />
  <path d="M8 10v8h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileBadge2;
impl IconShape for LdFileBadge2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <circle cx="12" cy="10" r="3" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m14 12.5 1 5.5-3-1-3 1 1-5.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileBadge;
impl IconShape for LdFileBadge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22h6a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M5 17a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
  <path d="M7 16.5 8 22l-3-1-3 1 1-5.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileBarChart2;
impl IconShape for LdFileBarChart2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M8 18v-1" />
  <path d="M12 18v-6" />
  <path d="M16 18v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileBarChart;
impl IconShape for LdFileBarChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M8 18v-2" />
  <path d="M12 18v-4" />
  <path d="M16 18v-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileBox;
impl IconShape for LdFileBox {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.5 22H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M3 13.1a2 2 0 0 0-1 1.76v3.24a2 2 0 0 0 .97 1.78L6 21.7a2 2 0 0 0 2.03.01L11 19.9a2 2 0 0 0 1-1.76V14.9a2 2 0 0 0-.97-1.78L8 11.3a2 2 0 0 0-2.03-.01Z" />
  <path d="M7 17v5" />
  <path d="M11.7 14.2 7 17l-4.7-2.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileCheck2;
impl IconShape for LdFileCheck2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m3 15 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileCheck;
impl IconShape for LdFileCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m9 15 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileClock;
impl IconShape for LdFileClock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="8" cy="16" r="6" />
  <path d="M9.5 17.5 8 16.25V14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileCode2;
impl IconShape for LdFileCode2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m5 12-3 3 3 3" />
  <path d="m9 18 3-3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileCode;
impl IconShape for LdFileCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m10 13-2 2 2 2" />
  <path d="m14 17 2-2-2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileCog;
impl IconShape for LdFileCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="6" cy="14" r="3" />
  <path d="M6 10v1" />
  <path d="M6 17v1" />
  <path d="M10 14H9" />
  <path d="M3 14H2" />
  <path d="m9 11-.88.88" />
  <path d="M3.88 16.12 3 17" />
  <path d="m9 17-.88-.88" />
  <path d="M3.88 11.88 3 11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileDiff;
impl IconShape for LdFileDiff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M9 10h6" />
  <path d="M12 13V7" />
  <path d="M9 17h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileDigit;
impl IconShape for LdFileDigit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <rect width="4" height="6" x="2" y="12" rx="2" />
  <path d="M10 12h2v6" />
  <path d="M10 18h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileDown;
impl IconShape for LdFileDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M12 18v-6" />
  <path d="m9 15 3 3 3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileHeart;
impl IconShape for LdFileHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v2" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M10.29 10.7a2.43 2.43 0 0 0-2.66-.52c-.29.12-.56.3-.78.53l-.35.34-.35-.34a2.43 2.43 0 0 0-2.65-.53c-.3.12-.56.3-.79.53-.95.94-1 2.53.2 3.74L6.5 18l3.6-3.55c1.2-1.21 1.14-2.8.19-3.74Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileImage;
impl IconShape for LdFileImage {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="10" cy="12" r="2" />
  <path d="m20 17-1.296-1.296a2.41 2.41 0 0 0-3.408 0L9 22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileInput;
impl IconShape for LdFileInput {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M2 15h10" />
  <path d="m9 18 3-3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileJson2;
impl IconShape for LdFileJson2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M4 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1" />
  <path d="M8 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileJson;
impl IconShape for LdFileJson {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1" />
  <path d="M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileKey2;
impl IconShape for LdFileKey2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v6" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="4" cy="16" r="2" />
  <path d="m10 10-4.5 4.5" />
  <path d="m9 11 1 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileKey;
impl IconShape for LdFileKey {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <circle cx="10" cy="16" r="2" />
  <path d="m16 10-4.5 4.5" />
  <path d="m15 11 1 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileLineChart;
impl IconShape for LdFileLineChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m16 13-3.5 3.5-2-2L8 17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileLock2;
impl IconShape for LdFileLock2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v1" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <rect width="8" height="5" x="2" y="13" rx="1" />
  <path d="M8 13v-2a2 2 0 1 0-4 0v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileLock;
impl IconShape for LdFileLock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <rect width="8" height="6" x="8" y="12" rx="1" />
  <path d="M10 12v-2a2 2 0 1 1 4 0v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileMinus2;
impl IconShape for LdFileMinus2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M3 15h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileMinus;
impl IconShape for LdFileMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M9 15h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileMusic;
impl IconShape for LdFileMusic {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="14" cy="16" r="2" />
  <circle cx="6" cy="18" r="2" />
  <path d="M4 12.4V4a2 2 0 0 1 2-2h8.5L20 7.5V20a2 2 0 0 1-2 2h-7.5" />
  <path d="M8 18v-7.7L16 9v7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileOutput;
impl IconShape for LdFileOutput {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M4 7V4a2 2 0 0 1 2-2 2 2 0 0 0-2 2" />
  <path d="M4.063 20.999a2 2 0 0 0 2 1L18 22a2 2 0 0 0 2-2V7l-5-5H6" />
  <path d="m5 11-3 3" />
  <path d="m5 17-3-3h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilePenLine;
impl IconShape for LdFilePenLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18 5-3-3H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2" />
  <path d="M8 18h1" />
  <path d="M18.4 9.6a2 2 0 1 1 3 3L17 17l-4 1 1-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilePen;
impl IconShape for LdFilePen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22h6a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v10" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M10.4 12.6a2 2 0 1 1 3 3L8 21l-4 1 1-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilePieChart;
impl IconShape for LdFilePieChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M16 22h2a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3.5" />
  <path d="M4.017 11.512a6 6 0 1 0 8.466 8.475" />
  <path d="M8 16v-6a6 6 0 0 1 6 6z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilePlus2;
impl IconShape for LdFilePlus2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M3 15h6" />
  <path d="M6 12v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilePlus;
impl IconShape for LdFilePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M9 15h6" />
  <path d="M12 18v-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileQuestion;
impl IconShape for LdFileQuestion {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 17h.01" />
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7z" />
  <path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileScan;
impl IconShape for LdFileScan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 10V7l-5-5H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M16 14a2 2 0 0 0-2 2" />
  <path d="M20 14a2 2 0 0 1 2 2" />
  <path d="M20 22a2 2 0 0 0 2-2" />
  <path d="M16 22a2 2 0 0 1-2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileSearch2;
impl IconShape for LdFileSearch2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <circle cx="11.5" cy="14.5" r="2.5" />
  <path d="M13.3 16.3 15 18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileSearch;
impl IconShape for LdFileSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M4.268 21a2 2 0 0 0 1.727 1H18a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" />
  <path d="m9 18-1.5-1.5" />
  <circle cx="5" cy="14" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileSliders;
impl IconShape for LdFileSliders {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M8 12h8" />
  <path d="M10 11v2" />
  <path d="M8 17h8" />
  <path d="M14 16v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileSpreadsheet;
impl IconShape for LdFileSpreadsheet {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M8 13h2" />
  <path d="M14 13h2" />
  <path d="M8 17h2" />
  <path d="M14 17h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileStack;
impl IconShape for LdFileStack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 7h-3a2 2 0 0 1-2-2V2" />
  <path d="M21 6v6.5c0 .8-.7 1.5-1.5 1.5h-7c-.8 0-1.5-.7-1.5-1.5v-9c0-.8.7-1.5 1.5-1.5H17Z" />
  <path d="M7 8v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H15" />
  <path d="M3 12v8.8c0 .3.2.6.4.8.2.2.5.4.8.4H11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileSymlink;
impl IconShape for LdFileSymlink {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m10 18 3-3-3-3" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M4 11V4a2 2 0 0 1 2-2h9l5 5v13a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileTerminal;
impl IconShape for LdFileTerminal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m8 16 2-2-2-2" />
  <path d="M12 18h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileText;
impl IconShape for LdFileText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M10 9H8" />
  <path d="M16 13H8" />
  <path d="M16 17H8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileType2;
impl IconShape for LdFileType2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M2 13v-1h6v1" />
  <path d="M5 12v6" />
  <path d="M4 18h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileType;
impl IconShape for LdFileType {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M9 13v-1h6v1" />
  <path d="M12 12v6" />
  <path d="M11 18h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileUp;
impl IconShape for LdFileUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M12 12v6" />
  <path d="m15 15-3-3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileVideo2;
impl IconShape for LdFileVideo2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <rect width="8" height="6" x="2" y="12" rx="1" />
  <path d="m10 15.5 4 2.5v-6l-4 2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileVideo;
impl IconShape for LdFileVideo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m10 11 5 3-5 3v-6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileVolume2;
impl IconShape for LdFileVolume2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M8 15h.01" />
  <path d="M11.5 13.5a2.5 2.5 0 0 1 0 3" />
  <path d="M15 12a5 5 0 0 1 0 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileVolume;
impl IconShape for LdFileVolume {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 11a5 5 0 0 1 0 6" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="M4.268 21A2 2 0 0 0 6 22h12a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v3" />
  <path d="m7 10-3 2H2v4h2l3 2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileWarning;
impl IconShape for LdFileWarning {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M12 9v4" />
  <path d="M12 17h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileX2;
impl IconShape for LdFileX2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h14a2 2 0 0 0 2-2V7l-5-5H6a2 2 0 0 0-2 2v4" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m8 12.5-5 5" />
  <path d="m3 12.5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFileX;
impl IconShape for LdFileX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  <path d="m14.5 12.5-5 5" />
  <path d="m9.5 12.5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFile;
impl IconShape for LdFile {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
  <path d="M14 2v4a2 2 0 0 0 2 2h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFiles;
impl IconShape for LdFiles {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 7h-3a2 2 0 0 1-2-2V2" />
  <path d="M9 18a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h7l4 4v10a2 2 0 0 1-2 2Z" />
  <path d="M3 7.6v12.8A1.6 1.6 0 0 0 4.6 22h9.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilm;
impl IconShape for LdFilm {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7 3v18" />
  <path d="M3 7.5h4" />
  <path d="M3 12h18" />
  <path d="M3 16.5h4" />
  <path d="M17 3v18" />
  <path d="M17 7.5h4" />
  <path d="M17 16.5h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilterX;
impl IconShape for LdFilterX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.013 3H2l8 9.46V19l4 2v-8.54l.9-1.055" />
  <path d="m22 3-5 5" />
  <path d="m17 3 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFilter;
impl IconShape for LdFilter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFingerprint;
impl IconShape for LdFingerprint {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4" />
  <path d="M14 13.12c0 2.38 0 6.38-1 8.88" />
  <path d="M17.29 21.02c.12-.6.43-2.3.5-3.02" />
  <path d="M2 12a10 10 0 0 1 18-6" />
  <path d="M2 16h.01" />
  <path d="M21.8 16c.2-2 .131-5.354 0-6" />
  <path d="M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2" />
  <path d="M8.65 22c.21-.66.45-1.32.57-2" />
  <path d="M9 6.8a6 6 0 0 1 9 5.2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFireExtinguisher;
impl IconShape for LdFireExtinguisher {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 6.5V3a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3.5" />
  <path d="M9 18h8" />
  <path d="M18 3h-3" />
  <path d="M11 3a6 6 0 0 0-6 6v11" />
  <path d="M5 13h4" />
  <path d="M17 10a4 4 0 0 0-8 0v10a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFishOff;
impl IconShape for LdFishOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 12.47v.03m0-.5v.47m-.475 5.056A6.744 6.744 0 0 1 15 18c-3.56 0-7.56-2.53-8.5-6 .348-1.28 1.114-2.433 2.121-3.38m3.444-2.088A8.802 8.802 0 0 1 15 6c3.56 0 6.06 2.54 7 6-.309 1.14-.786 2.177-1.413 3.058" />
  <path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33m7.48-4.372A9.77 9.77 0 0 1 16 6.07m0 11.86a9.77 9.77 0 0 1-1.728-3.618" />
  <path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98M8.53 3h5.27a2 2 0 0 1 1.98 1.67l.23 1.4M2 2l20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFishSymbol;
impl IconShape for LdFishSymbol {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 16s9-15 20-4C11 23 2 8 2 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFish;
impl IconShape for LdFish {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.47-3.44 6-7 6s-7.56-2.53-8.5-6Z" />
  <path d="M18 12v.5" />
  <path d="M16 17.93a9.77 9.77 0 0 1 0-11.86" />
  <path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33" />
  <path d="M10.46 7.26C10.2 5.88 9.17 4.24 8 3h5.8a2 2 0 0 1 1.98 1.67l.23 1.4" />
  <path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlagOff;
impl IconShape for LdFlagOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2c3 0 5 2 8 2s4-1 4-1v11" />
  <path d="M4 22V4" />
  <path d="M4 15s1-1 4-1 5 2 8 2" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlagTriangleLeft;
impl IconShape for LdFlagTriangleLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 22V2L7 7l10 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlagTriangleRight;
impl IconShape for LdFlagTriangleRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 22V2l10 5-10 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlag;
impl IconShape for LdFlag {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z" />
  <line x1="4" x2="4" y1="22" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlameKindling;
impl IconShape for LdFlameKindling {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2c1 3 2.5 3.5 3.5 4.5A5 5 0 0 1 17 10a5 5 0 1 1-10 0c0-.3 0-.6.1-.9a2 2 0 1 0 3.3-2C8 4.5 11 2 12 2Z" />
  <path d="m5 22 14-4" />
  <path d="m5 18 14 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlame;
impl IconShape for LdFlame {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlashlightOff;
impl IconShape for LdFlashlightOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4" />
  <path d="M7 2h11v4c0 2-2 2-2 4v1" />
  <line x1="11" x2="18" y1="6" y2="6" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlashlight;
impl IconShape for LdFlashlight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 6c0 2-2 2-2 4v10a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2V10c0-2-2-2-2-4V2h12z" />
  <line x1="6" x2="18" y1="6" y2="6" />
  <line x1="12" x2="12" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlaskConicalOff;
impl IconShape for LdFlaskConicalOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 10 4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-1.272-2.542" />
  <path d="M10 2v2.343" />
  <path d="M14 2v6.343" />
  <path d="M8.5 2h7" />
  <path d="M7 16h9" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlaskConical;
impl IconShape for LdFlaskConical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 2v7.527a2 2 0 0 1-.211.896L4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-5.069-10.127A2 2 0 0 1 14 9.527V2" />
  <path d="M8.5 2h7" />
  <path d="M7 16h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlaskRound;
impl IconShape for LdFlaskRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 2v7.31" />
  <path d="M14 9.3V1.99" />
  <path d="M8.5 2h7" />
  <path d="M14 9.3a6.5 6.5 0 1 1-4 0" />
  <path d="M5.52 16h12.96" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlipHorizontal2;
impl IconShape for LdFlipHorizontal2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 7 5 5-5 5V7" />
  <path d="m21 7-5 5 5 5V7" />
  <path d="M12 20v2" />
  <path d="M12 14v2" />
  <path d="M12 8v2" />
  <path d="M12 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlipHorizontal;
impl IconShape for LdFlipHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3" />
  <path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3" />
  <path d="M12 20v2" />
  <path d="M12 14v2" />
  <path d="M12 8v2" />
  <path d="M12 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlipVertical2;
impl IconShape for LdFlipVertical2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 3-5 5-5-5h10" />
  <path d="m17 21-5-5-5 5h10" />
  <path d="M4 12H2" />
  <path d="M10 12H8" />
  <path d="M16 12h-2" />
  <path d="M22 12h-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlipVertical;
impl IconShape for LdFlipVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3" />
  <path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3" />
  <path d="M4 12H2" />
  <path d="M10 12H8" />
  <path d="M16 12h-2" />
  <path d="M22 12h-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlower2;
impl IconShape for LdFlower2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1" />
  <circle cx="12" cy="8" r="2" />
  <path d="M12 10v12" />
  <path d="M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z" />
  <path d="M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFlower;
impl IconShape for LdFlower {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="3" />
  <path d="M12 16.5A4.5 4.5 0 1 1 7.5 12 4.5 4.5 0 1 1 12 7.5a4.5 4.5 0 1 1 4.5 4.5 4.5 4.5 0 1 1-4.5 4.5" />
  <path d="M12 7.5V9" />
  <path d="M7.5 12H9" />
  <path d="M16.5 12H15" />
  <path d="M12 16.5V15" />
  <path d="m8 8 1.88 1.88" />
  <path d="M14.12 9.88 16 8" />
  <path d="m8 16 1.88-1.88" />
  <path d="M14.12 14.12 16 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFocus;
impl IconShape for LdFocus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="3" />
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFoldHorizontal;
impl IconShape for LdFoldHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12h6" />
  <path d="M22 12h-6" />
  <path d="M12 2v2" />
  <path d="M12 8v2" />
  <path d="M12 14v2" />
  <path d="M12 20v2" />
  <path d="m19 9-3 3 3 3" />
  <path d="m5 15 3-3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFoldVertical;
impl IconShape for LdFoldVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22v-6" />
  <path d="M12 8V2" />
  <path d="M4 12H2" />
  <path d="M10 12H8" />
  <path d="M16 12h-2" />
  <path d="M22 12h-2" />
  <path d="m15 19-3-3-3 3" />
  <path d="m15 5-3 3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderArchive;
impl IconShape for LdFolderArchive {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="15" cy="19" r="2" />
  <path d="M20.9 19.8A2 2 0 0 0 22 18V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2h5.1" />
  <path d="M15 11v-1" />
  <path d="M15 17v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderCheck;
impl IconShape for LdFolderCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="m9 13 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderClock;
impl IconShape for LdFolderClock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="16" cy="16" r="6" />
  <path d="M7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2" />
  <path d="M16 14v2l1 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderClosed;
impl IconShape for LdFolderClosed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="M2 10h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderCog;
impl IconShape for LdFolderCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="18" r="3" />
  <path d="M10.3 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.3" />
  <path d="m21.7 19.4-.9-.3" />
  <path d="m15.2 16.9-.9-.3" />
  <path d="m16.6 21.7.3-.9" />
  <path d="m19.1 15.2.3-.9" />
  <path d="m19.6 21.7-.4-1" />
  <path d="m16.8 15.3-.4-1" />
  <path d="m14.3 19.6 1-.4" />
  <path d="m20.7 16.8 1-.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderDot;
impl IconShape for LdFolderDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
  <circle cx="12" cy="13" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderDown;
impl IconShape for LdFolderDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="M12 10v6" />
  <path d="m15 13-3 3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderGit2;
impl IconShape for LdFolderGit2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5" />
  <circle cx="13" cy="12" r="2" />
  <path d="M18 19c-2.8 0-5-2.2-5-5v8" />
  <circle cx="20" cy="19" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderGit;
impl IconShape for LdFolderGit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="13" r="2" />
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="M14 13h3" />
  <path d="M7 13h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderHeart;
impl IconShape for LdFolderHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v1.5" />
  <path d="M13.9 17.45c-1.2-1.2-1.14-2.8-.2-3.73a2.43 2.43 0 0 1 3.44 0l.36.34.34-.34a2.43 2.43 0 0 1 3.45-.01v0c.95.95 1 2.53-.2 3.74L17.5 21Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderInput;
impl IconShape for LdFolderInput {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-1" />
  <path d="M2 13h10" />
  <path d="m9 16 3-3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderKanban;
impl IconShape for LdFolderKanban {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
  <path d="M8 10v4" />
  <path d="M12 10v2" />
  <path d="M16 10v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderKey;
impl IconShape for LdFolderKey {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="16" cy="20" r="2" />
  <path d="M10 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v2" />
  <path d="m22 14-4.5 4.5" />
  <path d="m21 15 1 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderLock;
impl IconShape for LdFolderLock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="5" x="14" y="17" rx="1" />
  <path d="M10 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v2.5" />
  <path d="M20 17v-2a2 2 0 1 0-4 0v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderMinus;
impl IconShape for LdFolderMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 13h6" />
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderOpenDot;
impl IconShape for LdFolderOpenDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2" />
  <circle cx="14" cy="15" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderOpen;
impl IconShape for LdFolderOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderOutput;
impl IconShape for LdFolderOutput {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 7.5V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-1.5" />
  <path d="M2 13h10" />
  <path d="m5 10-3 3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderPen;
impl IconShape for LdFolderPen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.4 10.6a2 2 0 0 1 3 3L6 19l-4 1 1-4Z" />
  <path d="M2 11.5V5a2 2 0 0 1 2-2h3.9c.7 0 1.3.3 1.7.9l.8 1.2c.4.6 1 .9 1.7.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-9.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderPlus;
impl IconShape for LdFolderPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 10v6" />
  <path d="M9 13h6" />
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderRoot;
impl IconShape for LdFolderRoot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
  <circle cx="12" cy="13" r="2" />
  <path d="M12 15v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderSearch2;
impl IconShape for LdFolderSearch2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11.5" cy="12.5" r="2.5" />
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="M13.3 14.3 15 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderSearch;
impl IconShape for LdFolderSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="17" cy="17" r="3" />
  <path d="M10.7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v4.1" />
  <path d="m21 21-1.5-1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderSymlink;
impl IconShape for LdFolderSymlink {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h7" />
  <path d="m8 16 3-3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderSync;
impl IconShape for LdFolderSync {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v.5" />
  <path d="M12 10v4h4" />
  <path d="m12 14 1.535-1.605a5 5 0 0 1 8 1.5" />
  <path d="M22 22v-4h-4" />
  <path d="m22 18-1.535 1.605a5 5 0 0 1-8-1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderTree;
impl IconShape for LdFolderTree {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 10a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-2.5a1 1 0 0 1-.8-.4l-.9-1.2A1 1 0 0 0 15 3h-2a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z" />
  <path d="M20 21a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-2.9a1 1 0 0 1-.88-.55l-.42-.85a1 1 0 0 0-.92-.6H13a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z" />
  <path d="M3 5a2 2 0 0 0 2 2h3" />
  <path d="M3 3v13a2 2 0 0 0 2 2h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderUp;
impl IconShape for LdFolderUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="M12 10v6" />
  <path d="m9 13 3-3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolderX;
impl IconShape for LdFolderX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
  <path d="m9.5 10.5 5 5" />
  <path d="m14.5 10.5-5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolder;
impl IconShape for LdFolder {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFolders;
impl IconShape for LdFolders {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 17a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3.9a2 2 0 0 1-1.69-.9l-.81-1.2a2 2 0 0 0-1.67-.9H8a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2Z" />
  <path d="M2 8v11a2 2 0 0 0 2 2h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFootprints;
impl IconShape for LdFootprints {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z" />
  <path d="M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z" />
  <path d="M16 17h4" />
  <path d="M4 13h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdForklift;
impl IconShape for LdForklift {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 12H5a2 2 0 0 0-2 2v5" />
  <circle cx="13" cy="19" r="2" />
  <circle cx="5" cy="19" r="2" />
  <path d="M8 19h3m5-17v17h6M6 12V7c0-1.1.9-2 2-2h3l5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdForward;
impl IconShape for LdForward {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="15 17 20 12 15 7" />
  <path d="M4 18v-2a4 4 0 0 1 4-4h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFrame;
impl IconShape for LdFrame {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="22" x2="2" y1="6" y2="6" />
  <line x1="22" x2="2" y1="18" y2="18" />
  <line x1="6" x2="6" y1="2" y2="22" />
  <line x1="18" x2="18" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFramer;
impl IconShape for LdFramer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 16V9h14V2H5l14 14h-7m-7 0 7 7v-7m-7 0h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFrown;
impl IconShape for LdFrown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M16 16s-1.5-2-4-2-4 2-4 2" />
  <line x1="9" x2="9.01" y1="9" y2="9" />
  <line x1="15" x2="15.01" y1="9" y2="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFuel;
impl IconShape for LdFuel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="3" x2="15" y1="22" y2="22" />
  <line x1="4" x2="14" y1="9" y2="9" />
  <path d="M14 22V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v18" />
  <path d="M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 2 2h0a2 2 0 0 0 2-2V9.83a2 2 0 0 0-.59-1.42L18 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdFullscreen;
impl IconShape for LdFullscreen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <rect width="10" height="8" x="7" y="8" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGalleryHorizontalEnd;
impl IconShape for LdGalleryHorizontalEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 7v10" />
  <path d="M6 5v14" />
  <rect width="12" height="18" x="10" y="3" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGalleryHorizontal;
impl IconShape for LdGalleryHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 3v18" />
  <rect width="12" height="18" x="6" y="3" rx="2" />
  <path d="M22 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGalleryThumbnails;
impl IconShape for LdGalleryThumbnails {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="14" x="3" y="3" rx="2" />
  <path d="M4 21h1" />
  <path d="M9 21h1" />
  <path d="M14 21h1" />
  <path d="M19 21h1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGalleryVerticalEnd;
impl IconShape for LdGalleryVerticalEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 2h10" />
  <path d="M5 6h14" />
  <rect width="18" height="12" x="3" y="10" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGalleryVertical;
impl IconShape for LdGalleryVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 2h18" />
  <rect width="18" height="12" x="3" y="6" rx="2" />
  <path d="M3 22h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGamepad2;
impl IconShape for LdGamepad2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="6" x2="10" y1="11" y2="11" />
  <line x1="8" x2="8" y1="9" y2="13" />
  <line x1="15" x2="15.01" y1="12" y2="12" />
  <line x1="18" x2="18.01" y1="10" y2="10" />
  <path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGamepad;
impl IconShape for LdGamepad {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="6" x2="10" y1="12" y2="12" />
  <line x1="8" x2="8" y1="10" y2="14" />
  <line x1="15" x2="15.01" y1="13" y2="13" />
  <line x1="18" x2="18.01" y1="11" y2="11" />
  <rect width="20" height="12" x="2" y="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGanttChart;
impl IconShape for LdGanttChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 6h10" />
  <path d="M6 12h9" />
  <path d="M11 18h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGauge;
impl IconShape for LdGauge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 14 4-4" />
  <path d="M3.34 19a10 10 0 1 1 17.32 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGavel;
impl IconShape for LdGavel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m14.5 12.5-8 8a2.119 2.119 0 1 1-3-3l8-8" />
  <path d="m16 16 6-6" />
  <path d="m8 8 6-6" />
  <path d="m9 7 8 8" />
  <path d="m21 11-8-8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGem;
impl IconShape for LdGem {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 3h12l4 6-10 13L2 9Z" />
  <path d="M11 3 8 9l4 13 4-13-3-6" />
  <path d="M2 9h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGhost;
impl IconShape for LdGhost {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 10h.01" />
  <path d="M15 10h.01" />
  <path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGift;
impl IconShape for LdGift {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="3" y="8" width="18" height="4" rx="1" />
  <path d="M12 8v13" />
  <path d="M19 12v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-7" />
  <path d="M7.5 8a2.5 2.5 0 0 1 0-5A4.8 8 0 0 1 12 8a4.8 8 0 0 1 4.5-5 2.5 2.5 0 0 1 0 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitBranchPlus;
impl IconShape for LdGitBranchPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 3v12" />
  <path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
  <path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
  <path d="M15 6a9 9 0 0 0-9 9" />
  <path d="M18 15v6" />
  <path d="M21 18h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitBranch;
impl IconShape for LdGitBranch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="6" x2="6" y1="3" y2="15" />
  <circle cx="18" cy="6" r="3" />
  <circle cx="6" cy="18" r="3" />
  <path d="M18 9a9 9 0 0 1-9 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitCommitHorizontal;
impl IconShape for LdGitCommitHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="3" />
  <line x1="3" x2="9" y1="12" y2="12" />
  <line x1="15" x2="21" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitCommitVertical;
impl IconShape for LdGitCommitVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3v6" />
  <circle cx="12" cy="12" r="3" />
  <path d="M12 15v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitCompareArrows;
impl IconShape for LdGitCompareArrows {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="5" cy="6" r="3" />
  <path d="M12 6h5a2 2 0 0 1 2 2v7" />
  <path d="m15 9-3-3 3-3" />
  <circle cx="19" cy="18" r="3" />
  <path d="M12 18H7a2 2 0 0 1-2-2V9" />
  <path d="m9 15 3 3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitCompare;
impl IconShape for LdGitCompare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="18" r="3" />
  <circle cx="6" cy="6" r="3" />
  <path d="M13 6h3a2 2 0 0 1 2 2v7" />
  <path d="M11 18H8a2 2 0 0 1-2-2V9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitFork;
impl IconShape for LdGitFork {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="18" r="3" />
  <circle cx="6" cy="6" r="3" />
  <circle cx="18" cy="6" r="3" />
  <path d="M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9" />
  <path d="M12 12v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitGraph;
impl IconShape for LdGitGraph {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="5" cy="6" r="3" />
  <path d="M5 9v6" />
  <circle cx="5" cy="18" r="3" />
  <path d="M12 3v18" />
  <circle cx="19" cy="6" r="3" />
  <path d="M16 15.7A9 9 0 0 0 19 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitMerge;
impl IconShape for LdGitMerge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="18" r="3" />
  <circle cx="6" cy="6" r="3" />
  <path d="M6 21V9a9 9 0 0 0 9 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitPullRequestArrow;
impl IconShape for LdGitPullRequestArrow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="5" cy="6" r="3" />
  <path d="M5 9v12" />
  <circle cx="19" cy="18" r="3" />
  <path d="m15 9-3-3 3-3" />
  <path d="M12 6h5a2 2 0 0 1 2 2v7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitPullRequestClosed;
impl IconShape for LdGitPullRequestClosed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="6" r="3" />
  <path d="M6 9v12" />
  <path d="m21 3-6 6" />
  <path d="m21 9-6-6" />
  <path d="M18 11.5V15" />
  <circle cx="18" cy="18" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitPullRequestCreateArrow;
impl IconShape for LdGitPullRequestCreateArrow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="5" cy="6" r="3" />
  <path d="M5 9v12" />
  <path d="m15 9-3-3 3-3" />
  <path d="M12 6h5a2 2 0 0 1 2 2v3" />
  <path d="M19 15v6" />
  <path d="M22 18h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitPullRequestCreate;
impl IconShape for LdGitPullRequestCreate {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="6" r="3" />
  <path d="M6 9v12" />
  <path d="M13 6h3a2 2 0 0 1 2 2v3" />
  <path d="M18 15v6" />
  <path d="M21 18h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitPullRequestDraft;
impl IconShape for LdGitPullRequestDraft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="18" r="3" />
  <circle cx="6" cy="6" r="3" />
  <path d="M18 6V5" />
  <path d="M18 11v-1" />
  <line x1="6" x2="6" y1="9" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitPullRequest;
impl IconShape for LdGitPullRequest {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="18" r="3" />
  <circle cx="6" cy="6" r="3" />
  <path d="M13 6h3a2 2 0 0 1 2 2v7" />
  <line x1="6" x2="6" y1="9" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGithub;
impl IconShape for LdGithub {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
  <path d="M9 18c-4.51 2-5-2-7-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGitlab;
impl IconShape for LdGitlab {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m22 13.29-3.33-10a.42.42 0 0 0-.14-.18.38.38 0 0 0-.22-.11.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18l-2.26 6.67H8.32L6.1 3.26a.42.42 0 0 0-.1-.18.38.38 0 0 0-.26-.08.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18L2 13.29a.74.74 0 0 0 .27.83L12 21l9.69-6.88a.71.71 0 0 0 .31-.83Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGlassWater;
impl IconShape for LdGlassWater {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.2 22H8.8a2 2 0 0 1-2-1.79L5 3h14l-1.81 17.21A2 2 0 0 1 15.2 22Z" />
  <path d="M6 12a5 5 0 0 1 6 0 5 5 0 0 0 6 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGlasses;
impl IconShape for LdGlasses {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="15" r="4" />
  <circle cx="18" cy="15" r="4" />
  <path d="M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2" />
  <path d="M2.5 13 5 7c.7-1.3 1.4-2 3-2" />
  <path d="M21.5 13 19 7c-.7-1.3-1.5-2-3-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGlobeLock;
impl IconShape for LdGlobeLock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.686 15A14.5 14.5 0 0 1 12 22a14.5 14.5 0 0 1 0-20 10 10 0 1 0 9.542 13" />
  <path d="M2 12h8.5" />
  <path d="M20 6V4a2 2 0 1 0-4 0v2" />
  <rect width="8" height="5" x="14" y="6" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGlobe;
impl IconShape for LdGlobe {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
  <path d="M2 12h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGoal;
impl IconShape for LdGoal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 13V2l8 4-8 4" />
  <path d="M20.561 10.222a9 9 0 1 1-12.55-5.29" />
  <path d="M8.002 9.997a5 5 0 1 0 8.9 2.02" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGrab;
impl IconShape for LdGrab {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 11.5V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4" />
  <path d="M14 10V8a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2" />
  <path d="M10 9.9V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5" />
  <path d="M6 14v0a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" />
  <path d="M18 11v0a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGraduationCap;
impl IconShape for LdGraduationCap {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.42 10.922a1 1 0 0 0-.019-1.838L12.83 5.18a2 2 0 0 0-1.66 0L2.6 9.08a1 1 0 0 0 0 1.832l8.57 3.908a2 2 0 0 0 1.66 0z" />
  <path d="M22 10v6" />
  <path d="M6 12.5V16a6 3 0 0 0 12 0v-3.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGrape;
impl IconShape for LdGrape {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 5V2l-5.89 5.89" />
  <circle cx="16.6" cy="15.89" r="3" />
  <circle cx="8.11" cy="7.4" r="3" />
  <circle cx="12.35" cy="11.65" r="3" />
  <circle cx="13.91" cy="5.85" r="3" />
  <circle cx="18.15" cy="10.09" r="3" />
  <circle cx="6.56" cy="13.2" r="3" />
  <circle cx="10.8" cy="17.44" r="3" />
  <circle cx="5" cy="19" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGrid2x2;
impl IconShape for LdGrid2x2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 12h18" />
  <path d="M12 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGrid3x3;
impl IconShape for LdGrid3x3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9h18" />
  <path d="M3 15h18" />
  <path d="M9 3v18" />
  <path d="M15 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGripHorizontal;
impl IconShape for LdGripHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="9" r="1" />
  <circle cx="19" cy="9" r="1" />
  <circle cx="5" cy="9" r="1" />
  <circle cx="12" cy="15" r="1" />
  <circle cx="19" cy="15" r="1" />
  <circle cx="5" cy="15" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGripVertical;
impl IconShape for LdGripVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="9" cy="12" r="1" />
  <circle cx="9" cy="5" r="1" />
  <circle cx="9" cy="19" r="1" />
  <circle cx="15" cy="12" r="1" />
  <circle cx="15" cy="5" r="1" />
  <circle cx="15" cy="19" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGrip;
impl IconShape for LdGrip {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="5" r="1" />
  <circle cx="19" cy="5" r="1" />
  <circle cx="5" cy="5" r="1" />
  <circle cx="12" cy="12" r="1" />
  <circle cx="19" cy="12" r="1" />
  <circle cx="5" cy="12" r="1" />
  <circle cx="12" cy="19" r="1" />
  <circle cx="19" cy="19" r="1" />
  <circle cx="5" cy="19" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGroup;
impl IconShape for LdGroup {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5c0-1.1.9-2 2-2h2" />
  <path d="M17 3h2c1.1 0 2 .9 2 2v2" />
  <path d="M21 17v2c0 1.1-.9 2-2 2h-2" />
  <path d="M7 21H5c-1.1 0-2-.9-2-2v-2" />
  <rect width="7" height="5" x="7" y="7" rx="1" />
  <rect width="7" height="5" x="10" y="12" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdGuitar;
impl IconShape for LdGuitar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m20 7 1.7-1.7a1 1 0 0 0 0-1.4l-1.6-1.6a1 1 0 0 0-1.4 0L17 4v3Z" />
  <path d="m17 7-5.1 5.1" />
  <circle cx="11.5" cy="12.5" r=".5" fill="currentColor" />
  <path d="M6 12a2 2 0 0 0 1.8-1.2l.4-.9C8.7 8.8 9.8 8 11 8c2.8 0 5 2.2 5 5 0 1.2-.8 2.3-1.9 2.8l-.9.4A2 2 0 0 0 12 18a4 4 0 0 1-4 4c-3.3 0-6-2.7-6-6a4 4 0 0 1 4-4" />
  <path d="m6 16 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHam;
impl IconShape for LdHam {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.144 21.144A7.274 10.445 45 1 0 2.856 10.856" />
  <path d="M13.144 21.144A7.274 4.365 45 0 0 2.856 10.856a7.274 4.365 45 0 0 10.288 10.288" />
  <path d="M16.565 10.435 18.6 8.4a2.501 2.501 0 1 0 1.65-4.65 2.5 2.5 0 1 0-4.66 1.66l-2.024 2.025" />
  <path d="m8.5 16.5-1-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHammer;
impl IconShape for LdHammer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 12-8.373 8.373a1 1 0 1 1-3-3L12 9" />
  <path d="m18 15 4-4" />
  <path d="m21.5 11.5-1.914-1.914A2 2 0 0 1 19 8.172V7l-2.26-2.26a6 6 0 0 0-4.202-1.756L9 2.96l.92.82A6.18 6.18 0 0 1 12 8.4V10l2 2h1.172a2 2 0 0 1 1.414.586L18.5 14.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHandCoins;
impl IconShape for LdHandCoins {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 15h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 17" />
  <path d="m7 21 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9" />
  <path d="m2 16 6 6" />
  <circle cx="16" cy="9" r="2.9" />
  <circle cx="6" cy="5" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHandHeart;
impl IconShape for LdHandHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 14h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 16" />
  <path d="m7 20 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9" />
  <path d="m2 15 6 6" />
  <path d="M19.5 8.5c.7-.7 1.5-1.6 1.5-2.7A2.73 2.73 0 0 0 16 4a2.78 2.78 0 0 0-5 1.8c0 1.2.8 2 1.5 2.8L16 12Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHandHelping;
impl IconShape for LdHandHelping {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 12h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 14" />
  <path d="m7 18 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9" />
  <path d="m2 13 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHandMetal;
impl IconShape for LdHandMetal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 12.5V10a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1.4" />
  <path d="M14 11V9a2 2 0 1 0-4 0v2" />
  <path d="M10 10.5V5a2 2 0 1 0-4 0v9" />
  <path d="m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHandPlatter;
impl IconShape for LdHandPlatter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3V2" />
  <path d="M5 10a7.1 7.1 0 0 1 14 0" />
  <path d="M4 10h16" />
  <path d="M2 14h12a2 2 0 1 1 0 4h-2" />
  <path d="m15.4 17.4 3.2-2.8a2 2 0 0 1 2.8 2.9l-3.6 3.3c-.7.8-1.7 1.2-2.8 1.2h-4c-1.1 0-2.1-.4-2.8-1.2L5 18" />
  <path d="M5 14v7H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHand;
impl IconShape for LdHand {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 11V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" />
  <path d="M14 10V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2" />
  <path d="M10 10.5V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v8" />
  <path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHandshake;
impl IconShape for LdHandshake {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m11 17 2 2a1 1 0 1 0 3-3" />
  <path d="m14 14 2.5 2.5a1 1 0 1 0 3-3l-3.88-3.88a3 3 0 0 0-4.24 0l-.88.88a1 1 0 1 1-3-3l2.81-2.81a5.79 5.79 0 0 1 7.06-.87l.47.28a2 2 0 0 0 1.42.25L21 4" />
  <path d="m21 3 1 11h-2" />
  <path d="M3 3 2 14l6.5 6.5a1 1 0 1 0 3-3" />
  <path d="M3 4h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHardDriveDownload;
impl IconShape for LdHardDriveDownload {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v8" />
  <path d="m16 6-4 4-4-4" />
  <rect width="20" height="8" x="2" y="14" rx="2" />
  <path d="M6 18h.01" />
  <path d="M10 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHardDriveUpload;
impl IconShape for LdHardDriveUpload {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 6-4-4-4 4" />
  <path d="M12 2v8" />
  <rect width="20" height="8" x="2" y="14" rx="2" />
  <path d="M6 18h.01" />
  <path d="M10 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHardDrive;
impl IconShape for LdHardDrive {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="22" x2="2" y1="12" y2="12" />
  <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />
  <line x1="6" x2="6.01" y1="16" y2="16" />
  <line x1="10" x2="10.01" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHardHat;
impl IconShape for LdHardHat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 18a1 1 0 0 0 1 1h18a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v2z" />
  <path d="M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5" />
  <path d="M4 15v-3a6 6 0 0 1 6-6h0" />
  <path d="M14 6h0a6 6 0 0 1 6 6v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHash;
impl IconShape for LdHash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="4" x2="20" y1="9" y2="9" />
  <line x1="4" x2="20" y1="15" y2="15" />
  <line x1="10" x2="8" y1="3" y2="21" />
  <line x1="16" x2="14" y1="3" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHaze;
impl IconShape for LdHaze {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5.2 6.2 1.4 1.4" />
  <path d="M2 13h2" />
  <path d="M20 13h2" />
  <path d="m17.4 7.6 1.4-1.4" />
  <path d="M22 17H2" />
  <path d="M22 21H2" />
  <path d="M16 13a4 4 0 0 0-8 0" />
  <path d="M12 5V2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHdmiPort;
impl IconShape for LdHdmiPort {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 9a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1l2 2h12l2-2h1a1 1 0 0 0 1-1Z" />
  <path d="M7.5 12h9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading1;
impl IconShape for LdHeading1 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12h8" />
  <path d="M4 18V6" />
  <path d="M12 18V6" />
  <path d="m17 12 3-2v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading2;
impl IconShape for LdHeading2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12h8" />
  <path d="M4 18V6" />
  <path d="M12 18V6" />
  <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading3;
impl IconShape for LdHeading3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12h8" />
  <path d="M4 18V6" />
  <path d="M12 18V6" />
  <path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2" />
  <path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading4;
impl IconShape for LdHeading4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12h8" />
  <path d="M4 18V6" />
  <path d="M12 18V6" />
  <path d="M17 10v4h4" />
  <path d="M21 10v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading5;
impl IconShape for LdHeading5 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12h8" />
  <path d="M4 18V6" />
  <path d="M12 18V6" />
  <path d="M17 13v-3h4" />
  <path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading6;
impl IconShape for LdHeading6 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12h8" />
  <path d="M4 18V6" />
  <path d="M12 18V6" />
  <circle cx="19" cy="16" r="2" />
  <path d="M20 10c-2 2-3 3.5-3 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeading;
impl IconShape for LdHeading {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 12h12" />
  <path d="M6 20V4" />
  <path d="M18 20V4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeadphones;
impl IconShape for LdHeadphones {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeadset;
impl IconShape for LdHeadset {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 11h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-5Zm0 0a9 9 0 1 1 18 0m0 0v5a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3Z" />
  <path d="M21 16v2a4 4 0 0 1-4 4h-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeartCrack;
impl IconShape for LdHeartCrack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
  <path d="m12 13-1-1 2-2-3-3 2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeartHandshake;
impl IconShape for LdHeartHandshake {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
  <path d="M12 5 9.04 7.96a2.17 2.17 0 0 0 0 3.08v0c.82.82 2.13.85 3 .07l2.07-1.9a2.82 2.82 0 0 1 3.79 0l2.96 2.66" />
  <path d="m18 15-2-2" />
  <path d="m15 18-2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeartOff;
impl IconShape for LdHeartOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" y1="2" x2="22" y2="22" />
  <path d="M16.5 16.5 12 21l-7-7c-1.5-1.45-3-3.2-3-5.5a5.5 5.5 0 0 1 2.14-4.35" />
  <path d="M8.76 3.1c1.15.22 2.13.78 3.24 1.9 1.5-1.5 2.74-2 4.5-2A5.5 5.5 0 0 1 22 8.5c0 2.12-1.3 3.78-2.67 5.17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeartPulse;
impl IconShape for LdHeartPulse {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
  <path d="M3.22 12H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeart;
impl IconShape for LdHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHeater;
impl IconShape for LdHeater {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 8c2-3-2-3 0-6" />
  <path d="M15.5 8c2-3-2-3 0-6" />
  <path d="M6 10h.01" />
  <path d="M6 14h.01" />
  <path d="M10 16v-4" />
  <path d="M14 16v-4" />
  <path d="M18 16v-4" />
  <path d="M20 6a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3" />
  <path d="M5 20v2" />
  <path d="M19 20v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHexagon;
impl IconShape for LdHexagon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHighlighter;
impl IconShape for LdHighlighter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 11-6 6v3h9l3-3" />
  <path d="m22 12-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHistory;
impl IconShape for LdHistory {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
  <path d="M3 3v5h5" />
  <path d="M12 7v5l4 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHome;
impl IconShape for LdHome {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
  <polyline points="9 22 9 12 15 12 15 22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHopOff;
impl IconShape for LdHopOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.82 16.12c1.69.6 3.91.79 5.18.85.28.01.53-.09.7-.27" />
  <path d="M11.14 20.57c.52.24 2.44 1.12 4.08 1.37.46.06.86-.25.9-.71.12-1.52-.3-3.43-.5-4.28" />
  <path d="M16.13 21.05c1.65.63 3.68.84 4.87.91a.9.9 0 0 0 .7-.26" />
  <path d="M17.99 5.52a20.83 20.83 0 0 1 3.15 4.5.8.8 0 0 1-.68 1.13c-1.17.1-2.5.02-3.9-.25" />
  <path d="M20.57 11.14c.24.52 1.12 2.44 1.37 4.08.04.3-.08.59-.31.75" />
  <path d="M4.93 4.93a10 10 0 0 0-.67 13.4c.35.43.96.4 1.17-.12.69-1.71 1.07-5.07 1.07-6.71 1.34.45 3.1.9 4.88.62a.85.85 0 0 0 .48-.24" />
  <path d="M5.52 17.99c1.05.95 2.91 2.42 4.5 3.15a.8.8 0 0 0 1.13-.68c.2-2.34-.33-5.3-1.57-8.28" />
  <path d="M8.35 2.68a10 10 0 0 1 9.98 1.58c.43.35.4.96-.12 1.17-1.5.6-4.3.98-6.07 1.05" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHop;
impl IconShape for LdHop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.82 16.12c1.69.6 3.91.79 5.18.85.55.03 1-.42.97-.97-.06-1.27-.26-3.5-.85-5.18" />
  <path d="M11.5 6.5c1.64 0 5-.38 6.71-1.07.52-.2.55-.82.12-1.17A10 10 0 0 0 4.26 18.33c.35.43.96.4 1.17-.12.69-1.71 1.07-5.07 1.07-6.71 1.34.45 3.1.9 4.88.62a.88.88 0 0 0 .73-.74c.3-2.14-.15-3.5-.61-4.88" />
  <path d="M15.62 16.95c.2.85.62 2.76.5 4.28a.77.77 0 0 1-.9.7 16.64 16.64 0 0 1-4.08-1.36" />
  <path d="M16.13 21.05c1.65.63 3.68.84 4.87.91a.9.9 0 0 0 .96-.96 17.68 17.68 0 0 0-.9-4.87" />
  <path d="M16.94 15.62c.86.2 2.77.62 4.29.5a.77.77 0 0 0 .7-.9 16.64 16.64 0 0 0-1.36-4.08" />
  <path d="M17.99 5.52a20.82 20.82 0 0 1 3.15 4.5.8.8 0 0 1-.68 1.13c-2.33.2-5.3-.32-8.27-1.57" />
  <path d="M4.93 4.93 3 3a.7.7 0 0 1 0-1" />
  <path d="M9.58 12.18c1.24 2.98 1.77 5.95 1.57 8.28a.8.8 0 0 1-1.13.68 20.82 20.82 0 0 1-4.5-3.15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHospital;
impl IconShape for LdHospital {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 6v4" />
  <path d="M14 14h-4" />
  <path d="M14 18h-4" />
  <path d="M14 8h-4" />
  <path d="M18 12h2a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2" />
  <path d="M18 22V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHotel;
impl IconShape for LdHotel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 22v-6.57" />
  <path d="M12 11h.01" />
  <path d="M12 7h.01" />
  <path d="M14 15.43V22" />
  <path d="M15 16a5 5 0 0 0-6 0" />
  <path d="M16 11h.01" />
  <path d="M16 7h.01" />
  <path d="M8 11h.01" />
  <path d="M8 7h.01" />
  <rect x="4" y="2"  height="20" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdHourglass;
impl IconShape for LdHourglass {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 22h14" />
  <path d="M5 2h14" />
  <path d="M17 22v-4.172a2 2 0 0 0-.586-1.414L12 12l-4.414 4.414A2 2 0 0 0 7 17.828V22" />
  <path d="M7 2v4.172a2 2 0 0 0 .586 1.414L12 12l4.414-4.414A2 2 0 0 0 17 6.172V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIceCreamBowl;
impl IconShape for LdIceCreamBowl {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 17c5 0 8-2.69 8-6H4c0 3.31 3 6 8 6m-4 4h8m-4-3v3M5.14 11a3.5 3.5 0 1 1 6.71 0" />
  <path d="M12.14 11a3.5 3.5 0 1 1 6.71 0" />
  <path d="M15.5 6.5a3.5 3.5 0 1 0-7 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIceCreamCone;
impl IconShape for LdIceCreamCone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11" />
  <path d="M17 7A5 5 0 0 0 7 7" />
  <path d="M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImageDown;
impl IconShape for LdImageDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.814.014L6 21" />
  <path d="m14 19 3 3v-5.5" />
  <path d="m17 22 3-3" />
  <circle cx="9" cy="9" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImageMinus;
impl IconShape for LdImageMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 9v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" />
  <line x1="16" x2="22" y1="5" y2="5" />
  <circle cx="9" cy="9" r="2" />
  <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImageOff;
impl IconShape for LdImageOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="22" y1="2" y2="22" />
  <path d="M10.41 10.41a2 2 0 1 1-2.83-2.83" />
  <line x1="13.5" x2="6" y1="13.5" y2="21" />
  <line x1="18" x2="21" y1="12" y2="15" />
  <path d="M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59" />
  <path d="M21 15V5a2 2 0 0 0-2-2H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImagePlay;
impl IconShape for LdImagePlay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m11 16-5 5" />
  <path d="M11 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6.5" />
  <path d="M15.765 22a.5.5 0 0 1-.765-.424V13.38a.5.5 0 0 1 .765-.424l5.878 3.674a1 1 0 0 1 0 1.696z" />
  <circle cx="9" cy="9" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImagePlus;
impl IconShape for LdImagePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7" />
  <line x1="16" x2="22" y1="5" y2="5" />
  <line x1="19" x2="19" y1="2" y2="8" />
  <circle cx="9" cy="9" r="2" />
  <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImageUp;
impl IconShape for LdImageUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.814.014L6 21" />
  <path d="m14 19.5 3-3 3 3" />
  <path d="M17 22v-5.5" />
  <circle cx="9" cy="9" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImage;
impl IconShape for LdImage {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <circle cx="9" cy="9" r="2" />
  <path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImages;
impl IconShape for LdImages {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 22H4a2 2 0 0 1-2-2V6" />
  <path d="m22 13-1.296-1.296a2.41 2.41 0 0 0-3.408 0L11 18" />
  <circle cx="12" cy="8" r="2" />
  <rect   x="6" y="2" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdImport;
impl IconShape for LdImport {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3v12" />
  <path d="m8 11 4 4 4-4" />
  <path d="M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdInbox;
impl IconShape for LdInbox {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="22 12 16 12 14 15 10 15 8 12 2 12" />
  <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIndentDecrease;
impl IconShape for LdIndentDecrease {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="7 8 3 12 7 16" />
  <line x1="21" x2="11" y1="12" y2="12" />
  <line x1="21" x2="11" y1="6" y2="6" />
  <line x1="21" x2="11" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIndentIncrease;
impl IconShape for LdIndentIncrease {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="3 8 7 12 3 16" />
  <line x1="21" x2="11" y1="12" y2="12" />
  <line x1="21" x2="11" y1="6" y2="6" />
  <line x1="21" x2="11" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIndianRupee;
impl IconShape for LdIndianRupee {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 3h12" />
  <path d="M6 8h12" />
  <path d="m6 13 8.5 8" />
  <path d="M6 13h3" />
  <path d="M9 13c6.667 0 6.667-10 0-10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdInfinity;
impl IconShape for LdInfinity {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 12c-2-2.67-4-4-6-4a4 4 0 1 0 0 8c2 0 4-1.33 6-4Zm0 0c2 2.67 4 4 6 4a4 4 0 0 0 0-8c-2 0-4 1.33-6 4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdInfo;
impl IconShape for LdInfo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M12 16v-4" />
  <path d="M12 8h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdInspectionPanel;
impl IconShape for LdInspectionPanel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7 7h.01" />
  <path d="M17 7h.01" />
  <path d="M7 17h.01" />
  <path d="M17 17h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdInstagram;
impl IconShape for LdInstagram {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="20" x="2" y="2" rx="5" ry="5" />
  <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" />
  <line x1="17.5" x2="17.51" y1="6.5" y2="6.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdItalic;
impl IconShape for LdItalic {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="19" x2="10" y1="4" y2="4" />
  <line x1="14" x2="5" y1="20" y2="20" />
  <line x1="15" x2="9" y1="4" y2="20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIterationCcw;
impl IconShape for LdIterationCcw {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 10c0-4.4-3.6-8-8-8s-8 3.6-8 8 3.6 8 8 8h8" />
  <polyline points="16 14 20 18 16 22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdIterationCw;
impl IconShape for LdIterationCw {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 10c0-4.4 3.6-8 8-8s8 3.6 8 8-3.6 8-8 8H4" />
  <polyline points="8 22 4 18 8 14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdJapaneseYen;
impl IconShape for LdJapaneseYen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 9.5V21m0-11.5L6 3m6 6.5L18 3" />
  <path d="M6 15h12" />
  <path d="M6 11h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdJoystick;
impl IconShape for LdJoystick {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z" />
  <path d="M6 15v-2" />
  <path d="M12 15V9" />
  <circle cx="12" cy="6" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKanban;
impl IconShape for LdKanban {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 5v11" />
  <path d="M12 5v6" />
  <path d="M18 5v14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKeyRound;
impl IconShape for LdKeyRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4a6.5 6.5 0 1 0-4-4Z" />
  <circle cx="16.5" cy="7.5" r=".5" fill="currentColor" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKeySquare;
impl IconShape for LdKeySquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.4 2.7c.9-.9 2.5-.9 3.4 0l5.5 5.5c.9.9.9 2.5 0 3.4l-3.7 3.7c-.9.9-2.5.9-3.4 0L8.7 9.8c-.9-.9-.9-2.5 0-3.4Z" />
  <path d="m14 7 3 3" />
  <path d="M9.4 10.6 2 18v3c0 .6.4 1 1 1h4v-3h3v-3h2l1.4-1.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKey;
impl IconShape for LdKey {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="7.5" cy="15.5" r="5.5" />
  <path d="m21 2-9.6 9.6" />
  <path d="m15.5 7.5 3 3L22 7l-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKeyboardMusic;
impl IconShape for LdKeyboardMusic {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="M6 8h4" />
  <path d="M14 8h.01" />
  <path d="M18 8h.01" />
  <path d="M2 12h20" />
  <path d="M6 12v4" />
  <path d="M10 12v4" />
  <path d="M14 12v4" />
  <path d="M18 12v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKeyboardOff;
impl IconShape for LdKeyboardOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M 20 4 A2 2 0 0 1 22 6" />
  <path d="M 22 6 L 22 16.41" />
  <path d="M 7 16 L 16 16" />
  <path d="M 9.69 4 L 20 4" />
  <path d="M14 8h.01" />
  <path d="M18 8h.01" />
  <path d="m2 2 20 20" />
  <path d="M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2" />
  <path d="M6 8h.01" />
  <path d="M8 12h.01" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdKeyboard;
impl IconShape for LdKeyboard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 8h.01" />
  <path d="M12 12h.01" />
  <path d="M14 8h.01" />
  <path d="M16 12h.01" />
  <path d="M18 8h.01" />
  <path d="M6 8h.01" />
  <path d="M7 16h10" />
  <path d="M8 12h.01" />
  <rect width="20"  x="2" y="4" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLampCeiling;
impl IconShape for LdLampCeiling {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v5" />
  <path d="M6 7h12l4 9H2l4-9Z" />
  <path d="M9.17 16a3 3 0 1 0 5.66 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLampDesk;
impl IconShape for LdLampDesk {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m14 5-3 3 2 7 8-8-7-2Z" />
  <path d="m14 5-3 3-3-3 3-3 3 3Z" />
  <path d="M9.5 6.5 4 12l3 6" />
  <path d="M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLampFloor;
impl IconShape for LdLampFloor {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 2h6l3 7H6l3-7Z" />
  <path d="M12 9v13" />
  <path d="M9 22h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLampWallDown;
impl IconShape for LdLampWallDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 13h6l3 7H8l3-7Z" />
  <path d="M14 13V8a2 2 0 0 0-2-2H8" />
  <path d="M4 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H4v6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLampWallUp;
impl IconShape for LdLampWallUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 4h6l3 7H8l3-7Z" />
  <path d="M14 11v5a2 2 0 0 1-2 2H8" />
  <path d="M4 15h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H4v-6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLamp;
impl IconShape for LdLamp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2h8l4 10H4L8 2Z" />
  <path d="M12 12v6" />
  <path d="M8 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H8Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLandPlot;
impl IconShape for LdLandPlot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 8 6-3-6-3v10" />
  <path d="m8 11.99-5.5 3.14a1 1 0 0 0 0 1.74l8.5 4.86a2 2 0 0 0 2 0l8.5-4.86a1 1 0 0 0 0-1.74L16 12" />
  <path d="m6.49 12.85 11.02 6.3" />
  <path d="M17.51 12.85 6.5 19.15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLandmark;
impl IconShape for LdLandmark {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="3" x2="21" y1="22" y2="22" />
  <line x1="6" x2="6" y1="18" y2="11" />
  <line x1="10" x2="10" y1="18" y2="11" />
  <line x1="14" x2="14" y1="18" y2="11" />
  <line x1="18" x2="18" y1="18" y2="11" />
  <polygon points="12 2 20 7 4 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLanguages;
impl IconShape for LdLanguages {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5 8 6 6" />
  <path d="m4 14 6-6 2-3" />
  <path d="M2 5h12" />
  <path d="M7 2h1" />
  <path d="m22 22-5-10-5 10" />
  <path d="M14 18h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLaptopMinimal;
impl IconShape for LdLaptopMinimal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="12" x="3" y="4" rx="2" ry="2" />
  <line x1="2" x2="22" y1="20" y2="20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLaptop;
impl IconShape for LdLaptop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 16V7a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v9m16 0H4m16 0 1.28 2.55a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45L4 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLassoSelect;
impl IconShape for LdLassoSelect {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 22a5 5 0 0 1-2-4" />
  <path d="M7 16.93c.96.43 1.96.74 2.99.91" />
  <path d="M3.34 14A6.8 6.8 0 0 1 2 10c0-4.42 4.48-8 10-8s10 3.58 10 8a7.19 7.19 0 0 1-.33 2" />
  <path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
  <path d="M14.33 22h-.09a.35.35 0 0 1-.24-.32v-10a.34.34 0 0 1 .33-.34c.08 0 .15.03.21.08l7.34 6a.33.33 0 0 1-.21.59h-4.49l-2.57 3.85a.35.35 0 0 1-.28.14v0z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLasso;
impl IconShape for LdLasso {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 22a5 5 0 0 1-2-4" />
  <path d="M3.3 14A6.8 6.8 0 0 1 2 10c0-4.4 4.5-8 10-8s10 3.6 10 8-4.5 8-10 8a12 12 0 0 1-5-1" />
  <path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLaugh;
impl IconShape for LdLaugh {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" />
  <line x1="9" x2="9.01" y1="9" y2="9" />
  <line x1="15" x2="15.01" y1="9" y2="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayers2;
impl IconShape for LdLayers2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16.02 12 5.48 3.13a1 1 0 0 1 0 1.74L13 21.74a2 2 0 0 1-2 0l-8.5-4.87a1 1 0 0 1 0-1.74L7.98 12" />
  <path d="M13 13.74a2 2 0 0 1-2 0L2.5 8.87a1 1 0 0 1 0-1.74L11 2.26a2 2 0 0 1 2 0l8.5 4.87a1 1 0 0 1 0 1.74Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayers3;
impl IconShape for LdLayers3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" />
  <path d="m6.08 9.5-3.5 1.6a1 1 0 0 0 0 1.81l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9a1 1 0 0 0 0-1.83l-3.5-1.59" />
  <path d="m6.08 14.5-3.5 1.6a1 1 0 0 0 0 1.81l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9a1 1 0 0 0 0-1.83l-3.5-1.59" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayers;
impl IconShape for LdLayers {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" />
  <path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65" />
  <path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayoutDashboard;
impl IconShape for LdLayoutDashboard {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="9" x="3" y="3" rx="1" />
  <rect width="7" height="5" x="14" y="3" rx="1" />
  <rect width="7" height="9" x="14" y="12" rx="1" />
  <rect width="7" height="5" x="3" y="16" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayoutGrid;
impl IconShape for LdLayoutGrid {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="7" x="3" y="3" rx="1" />
  <rect width="7" height="7" x="14" y="3" rx="1" />
  <rect width="7" height="7" x="14" y="14" rx="1" />
  <rect width="7" height="7" x="3" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayoutList;
impl IconShape for LdLayoutList {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="7" x="3" y="3" rx="1" />
  <rect width="7" height="7" x="3" y="14" rx="1" />
  <path d="M14 4h7" />
  <path d="M14 9h7" />
  <path d="M14 15h7" />
  <path d="M14 20h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayoutPanelLeft;
impl IconShape for LdLayoutPanelLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="18" x="3" y="3" rx="1" />
  <rect width="7" height="7" x="14" y="3" rx="1" />
  <rect width="7" height="7" x="14" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayoutPanelTop;
impl IconShape for LdLayoutPanelTop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="7" x="3" y="3" rx="1" />
  <rect width="7" height="7" x="3" y="14" rx="1" />
  <rect width="7" height="7" x="14" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLayoutTemplate;
impl IconShape for LdLayoutTemplate {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="7" x="3" y="3" rx="1" />
  <rect width="9" height="7" x="3" y="14" rx="1" />
  <rect width="5" height="7" x="16" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLeaf;
impl IconShape for LdLeaf {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z" />
  <path d="M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLeafyGreen;
impl IconShape for LdLeafyGreen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 22c1.25-.987 2.27-1.975 3.9-2.2a5.56 5.56 0 0 1 3.8 1.5 4 4 0 0 0 6.187-2.353 3.5 3.5 0 0 0 3.69-5.116A3.5 3.5 0 0 0 20.95 8 3.5 3.5 0 1 0 16 3.05a3.5 3.5 0 0 0-5.831 1.373 3.5 3.5 0 0 0-5.116 3.69 4 4 0 0 0-2.348 6.155C3.499 15.42 4.409 16.712 4.2 18.1 3.926 19.743 3.014 20.732 2 22" />
  <path d="M2 22 17 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLibraryBig;
impl IconShape for LdLibraryBig {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="18" x="3" y="3" rx="1" />
  <path d="M7 3v18" />
  <path d="M20.4 18.9c.2.5-.1 1.1-.6 1.3l-1.9.7c-.5.2-1.1-.1-1.3-.6L11.1 5.1c-.2-.5.1-1.1.6-1.3l1.9-.7c.5-.2 1.1.1 1.3.6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLibrary;
impl IconShape for LdLibrary {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 6 4 14" />
  <path d="M12 6v14" />
  <path d="M8 8v12" />
  <path d="M4 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLifeBuoy;
impl IconShape for LdLifeBuoy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="m4.93 4.93 4.24 4.24" />
  <path d="m14.83 9.17 4.24-4.24" />
  <path d="m14.83 14.83 4.24 4.24" />
  <path d="m9.17 14.83-4.24 4.24" />
  <circle cx="12" cy="12" r="4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLigature;
impl IconShape for LdLigature {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 20V8c0-2.2 1.8-4 4-4 1.5 0 2.8.8 3.5 2" />
  <path d="M6 12h4" />
  <path d="M14 12h2v8" />
  <path d="M6 20h4" />
  <path d="M14 20h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLightbulbOff;
impl IconShape for LdLightbulbOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16.8 11.2c.8-.9 1.2-2 1.2-3.2a6 6 0 0 0-9.3-5" />
  <path d="m2 2 20 20" />
  <path d="M6.3 6.3a4.67 4.67 0 0 0 1.2 5.2c.7.7 1.3 1.5 1.5 2.5" />
  <path d="M9 18h6" />
  <path d="M10 22h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLightbulb;
impl IconShape for LdLightbulb {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" />
  <path d="M9 18h6" />
  <path d="M10 22h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLineChart;
impl IconShape for LdLineChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3v18h18" />
  <path d="m19 9-5 5-4-4-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLink2Off;
impl IconShape for LdLink2Off {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 17H7A5 5 0 0 1 7 7" />
  <path d="M15 7h2a5 5 0 0 1 4 8" />
  <line x1="8" x2="12" y1="12" y2="12" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLink2;
impl IconShape for LdLink2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 17H7A5 5 0 0 1 7 7h2" />
  <path d="M15 7h2a5 5 0 1 1 0 10h-2" />
  <line x1="8" x2="16" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLink;
impl IconShape for LdLink {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
  <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLinkedin;
impl IconShape for LdLinkedin {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
  <rect width="4" height="12" x="2" y="9" />
  <circle cx="4" cy="4" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListChecks;
impl IconShape for LdListChecks {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 17 2 2 4-4" />
  <path d="m3 7 2 2 4-4" />
  <path d="M13 6h8" />
  <path d="M13 12h8" />
  <path d="M13 18h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListCollapse;
impl IconShape for LdListCollapse {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 10 2.5-2.5L3 5" />
  <path d="m3 19 2.5-2.5L3 14" />
  <path d="M10 6h11" />
  <path d="M10 12h11" />
  <path d="M10 18h11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListEnd;
impl IconShape for LdListEnd {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 12H3" />
  <path d="M16 6H3" />
  <path d="M10 18H3" />
  <path d="M21 6v10a2 2 0 0 1-2 2h-5" />
  <path d="m16 16-2 2 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListFilter;
impl IconShape for LdListFilter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 6h18" />
  <path d="M7 12h10" />
  <path d="M10 18h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListMinus;
impl IconShape for LdListMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 12H3" />
  <path d="M16 6H3" />
  <path d="M16 18H3" />
  <path d="M21 12h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListMusic;
impl IconShape for LdListMusic {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15V6" />
  <path d="M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" />
  <path d="M12 12H3" />
  <path d="M16 6H3" />
  <path d="M12 18H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListOrdered;
impl IconShape for LdListOrdered {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="10" x2="21" y1="6" y2="6" />
  <line x1="10" x2="21" y1="12" y2="12" />
  <line x1="10" x2="21" y1="18" y2="18" />
  <path d="M4 6h1v4" />
  <path d="M4 10h2" />
  <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListPlus;
impl IconShape for LdListPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 12H3" />
  <path d="M16 6H3" />
  <path d="M16 18H3" />
  <path d="M18 9v6" />
  <path d="M21 12h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListRestart;
impl IconShape for LdListRestart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 6H3" />
  <path d="M7 12H3" />
  <path d="M7 18H3" />
  <path d="M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14" />
  <path d="M11 10v4h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListStart;
impl IconShape for LdListStart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 12H3" />
  <path d="M16 18H3" />
  <path d="M10 6H3" />
  <path d="M21 18V8a2 2 0 0 0-2-2h-5" />
  <path d="m16 8-2-2 2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListTodo;
impl IconShape for LdListTodo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="3" y="5" width="6" height="6" rx="1" />
  <path d="m3 17 2 2 4-4" />
  <path d="M13 6h8" />
  <path d="M13 12h8" />
  <path d="M13 18h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListTree;
impl IconShape for LdListTree {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12h-8" />
  <path d="M21 6H8" />
  <path d="M21 18h-8" />
  <path d="M3 6v4c0 1.1.9 2 2 2h3" />
  <path d="M3 10v6c0 1.1.9 2 2 2h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListVideo;
impl IconShape for LdListVideo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 12H3" />
  <path d="M16 6H3" />
  <path d="M12 18H3" />
  <path d="m16 12 5 3-5 3v-6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdListX;
impl IconShape for LdListX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 12H3" />
  <path d="M16 6H3" />
  <path d="M16 18H3" />
  <path d="m19 10-4 4" />
  <path d="m15 10 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdList;
impl IconShape for LdList {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="8" x2="21" y1="6" y2="6" />
  <line x1="8" x2="21" y1="12" y2="12" />
  <line x1="8" x2="21" y1="18" y2="18" />
  <line x1="3" x2="3.01" y1="6" y2="6" />
  <line x1="3" x2="3.01" y1="12" y2="12" />
  <line x1="3" x2="3.01" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLoaderCircle;
impl IconShape for LdLoaderCircle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12a9 9 0 1 1-6.219-8.56" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLoader;
impl IconShape for LdLoader {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="12" y1="2" y2="6" />
  <line x1="12" x2="12" y1="18" y2="22" />
  <line x1="4.93" x2="7.76" y1="4.93" y2="7.76" />
  <line x1="16.24" x2="19.07" y1="16.24" y2="19.07" />
  <line x1="2" x2="6" y1="12" y2="12" />
  <line x1="18" x2="22" y1="12" y2="12" />
  <line x1="4.93" x2="7.76" y1="19.07" y2="16.24" />
  <line x1="16.24" x2="19.07" y1="7.76" y2="4.93" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLocateFixed;
impl IconShape for LdLocateFixed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="5" y1="12" y2="12" />
  <line x1="19" x2="22" y1="12" y2="12" />
  <line x1="12" x2="12" y1="2" y2="5" />
  <line x1="12" x2="12" y1="19" y2="22" />
  <circle cx="12" cy="12" r="7" />
  <circle cx="12" cy="12" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLocateOff;
impl IconShape for LdLocateOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="5" y1="12" y2="12" />
  <line x1="19" x2="22" y1="12" y2="12" />
  <line x1="12" x2="12" y1="2" y2="5" />
  <line x1="12" x2="12" y1="19" y2="22" />
  <path d="M7.11 7.11C5.83 8.39 5 10.1 5 12c0 3.87 3.13 7 7 7 1.9 0 3.61-.83 4.89-2.11" />
  <path d="M18.71 13.96c.19-.63.29-1.29.29-1.96 0-3.87-3.13-7-7-7-.67 0-1.33.1-1.96.29" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLocate;
impl IconShape for LdLocate {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="5" y1="12" y2="12" />
  <line x1="19" x2="22" y1="12" y2="12" />
  <line x1="12" x2="12" y1="2" y2="5" />
  <line x1="12" x2="12" y1="19" y2="22" />
  <circle cx="12" cy="12" r="7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLockKeyholeOpen;
impl IconShape for LdLockKeyholeOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="16" r="1" />
  <rect width="18" height="12" x="3" y="10" rx="2" />
  <path d="M7 10V7a5 5 0 0 1 9.33-2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLockKeyhole;
impl IconShape for LdLockKeyhole {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="16" r="1" />
  <rect x="3" y="10" width="18" height="12" rx="2" />
  <path d="M7 10V7a5 5 0 0 1 10 0v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLockOpen;
impl IconShape for LdLockOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
  <path d="M7 11V7a5 5 0 0 1 9.9-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLock;
impl IconShape for LdLock {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
  <path d="M7 11V7a5 5 0 0 1 10 0v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLogIn;
impl IconShape for LdLogIn {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" />
  <polyline points="10 17 15 12 10 7" />
  <line x1="15" x2="3" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLogOut;
impl IconShape for LdLogOut {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
  <polyline points="16 17 21 12 16 7" />
  <line x1="21" x2="9" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLollipop;
impl IconShape for LdLollipop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11" cy="11" r="8" />
  <path d="m21 21-4.3-4.3" />
  <path d="M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdLuggage;
impl IconShape for LdLuggage {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 20h0a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h0" />
  <path d="M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14" />
  <path d="M10 20h4" />
  <circle cx="16" cy="20" r="2" />
  <circle cx="8" cy="20" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMagnet;
impl IconShape for LdMagnet {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 15-4-4 6.75-6.77a7.79 7.79 0 0 1 11 11L13 22l-4-4 6.39-6.36a2.14 2.14 0 0 0-3-3L6 15" />
  <path d="m5 8 4 4" />
  <path d="m12 15 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailCheck;
impl IconShape for LdMailCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="m16 19 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailMinus;
impl IconShape for LdMailMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 15V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="M16 19h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailOpen;
impl IconShape for LdMailOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z" />
  <path d="m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailPlus;
impl IconShape for LdMailPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="M19 16v6" />
  <path d="M16 19h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailQuestion;
impl IconShape for LdMailQuestion {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="M18 15.28c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2" />
  <path d="M20 22v.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailSearch;
impl IconShape for LdMailSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6v0Z" />
  <circle cx="18" cy="18" r="3" />
  <path d="m22 22-1.5-1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailWarning;
impl IconShape for LdMailWarning {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="M20 14v4" />
  <path d="M20 22v.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailX;
impl IconShape for LdMailX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h9" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
  <path d="m17 17 4 4" />
  <path d="m21 17-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMail;
impl IconShape for LdMail {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMailbox;
impl IconShape for LdMailbox {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z" />
  <polyline points="15,9 18,9 18,11" />
  <path d="M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2v0" />
  <line x1="6" x2="7" y1="10" y2="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMails;
impl IconShape for LdMails {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="13" x="6" y="4" rx="2" />
  <path d="m22 7-7.1 3.78c-.57.3-1.23.3-1.8 0L6 7" />
  <path d="M2 8v11c0 1.1.9 2 2 2h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMapPinOff;
impl IconShape for LdMapPinOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5.43 5.43A8.06 8.06 0 0 0 4 10c0 6 8 12 8 12a29.94 29.94 0 0 0 5-5" />
  <path d="M19.18 13.52A8.66 8.66 0 0 0 20 10a8 8 0 0 0-8-8 7.88 7.88 0 0 0-3.52.82" />
  <path d="M9.13 9.13A2.78 2.78 0 0 0 9 10a3 3 0 0 0 3 3 2.78 2.78 0 0 0 .87-.13" />
  <path d="M14.9 9.25a3 3 0 0 0-2.15-2.16" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMapPin;
impl IconShape for LdMapPin {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" />
  <circle cx="12" cy="10" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMapPinned;
impl IconShape for LdMapPinned {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 8c0 4.5-6 9-6 9s-6-4.5-6-9a6 6 0 0 1 12 0" />
  <circle cx="12" cy="8" r="2" />
  <path d="M8.835 14H5a1 1 0 0 0-.9.7l-2 6c-.1.1-.1.2-.1.3 0 .6.4 1 1 1h18c.6 0 1-.4 1-1 0-.1 0-.2-.1-.3l-2-6a1 1 0 0 0-.9-.7h-3.835" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMap;
impl IconShape for LdMap {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.106 5.553a2 2 0 0 0 1.788 0l3.659-1.83A1 1 0 0 1 21 4.619v12.764a1 1 0 0 1-.553.894l-4.553 2.277a2 2 0 0 1-1.788 0l-4.212-2.106a2 2 0 0 0-1.788 0l-3.659 1.83A1 1 0 0 1 3 19.381V6.618a1 1 0 0 1 .553-.894l4.553-2.277a2 2 0 0 1 1.788 0z" />
  <path d="M15 5.764v15" />
  <path d="M9 3.236v15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMartini;
impl IconShape for LdMartini {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 22h8" />
  <path d="M12 11v11" />
  <path d="m19 3-7 8-7-8Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMaximize2;
impl IconShape for LdMaximize2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="15 3 21 3 21 9" />
  <polyline points="9 21 3 21 3 15" />
  <line x1="21" x2="14" y1="3" y2="10" />
  <line x1="3" x2="10" y1="21" y2="14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMaximize;
impl IconShape for LdMaximize {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3H5a2 2 0 0 0-2 2v3" />
  <path d="M21 8V5a2 2 0 0 0-2-2h-3" />
  <path d="M3 16v3a2 2 0 0 0 2 2h3" />
  <path d="M16 21h3a2 2 0 0 0 2-2v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMedal;
impl IconShape for LdMedal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15" />
  <path d="M11 12 5.12 2.2" />
  <path d="m13 12 5.88-9.8" />
  <path d="M8 7h8" />
  <circle cx="12" cy="17" r="5" />
  <path d="M12 18v-2h-.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMegaphoneOff;
impl IconShape for LdMegaphoneOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9.26 9.26 3 11v3l14.14 3.14" />
  <path d="M21 15.34V6l-7.31 2.03" />
  <path d="M11.6 16.8a3 3 0 1 1-5.8-1.6" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMegaphone;
impl IconShape for LdMegaphone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 11 18-5v12L3 14v-3z" />
  <path d="M11.6 16.8a3 3 0 1 1-5.8-1.6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMeh;
impl IconShape for LdMeh {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <line x1="8" x2="16" y1="15" y2="15" />
  <line x1="9" x2="9.01" y1="9" y2="9" />
  <line x1="15" x2="15.01" y1="9" y2="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMemoryStick;
impl IconShape for LdMemoryStick {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 19v-3" />
  <path d="M10 19v-3" />
  <path d="M14 19v-3" />
  <path d="M18 19v-3" />
  <path d="M8 11V9" />
  <path d="M16 11V9" />
  <path d="M12 11V9" />
  <path d="M2 15h20" />
  <path d="M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.1a2 2 0 0 0 0 3.837V17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-5.1a2 2 0 0 0 0-3.837Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMenu;
impl IconShape for LdMenu {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="4" x2="20" y1="12" y2="12" />
  <line x1="4" x2="20" y1="6" y2="6" />
  <line x1="4" x2="20" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMerge;
impl IconShape for LdMerge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8 6 4-4 4 4" />
  <path d="M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22" />
  <path d="m20 22-5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleCode;
impl IconShape for LdMessageCircleCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="m10 10-2 2 2 2" />
  <path d="m14 10 2 2-2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleDashed;
impl IconShape for LdMessageCircleDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.5 3.1c-.5 0-1-.1-1.5-.1s-1 .1-1.5.1" />
  <path d="M19.3 6.8a10.45 10.45 0 0 0-2.1-2.1" />
  <path d="M20.9 13.5c.1-.5.1-1 .1-1.5s-.1-1-.1-1.5" />
  <path d="M17.2 19.3a10.45 10.45 0 0 0 2.1-2.1" />
  <path d="M10.5 20.9c.5.1 1 .1 1.5.1s1-.1 1.5-.1" />
  <path d="M3.5 17.5 2 22l4.5-1.5" />
  <path d="M3.1 10.5c0 .5-.1 1-.1 1.5s.1 1 .1 1.5" />
  <path d="M6.8 4.7a10.45 10.45 0 0 0-2.1 2.1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleHeart;
impl IconShape for LdMessageCircleHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="M15.8 9.2a2.5 2.5 0 0 0-3.5 0l-.3.4-.35-.3a2.42 2.42 0 1 0-3.2 3.6l3.6 3.5 3.6-3.5c1.2-1.2 1.1-2.7.2-3.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleMore;
impl IconShape for LdMessageCircleMore {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="M8 12h.01" />
  <path d="M12 12h.01" />
  <path d="M16 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleOff;
impl IconShape for LdMessageCircleOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20.5 14.9A9 9 0 0 0 9.1 3.5" />
  <path d="m2 2 20 20" />
  <path d="M5.6 5.6C3 8.3 2.2 12.5 4 16l-2 6 6-2c3.4 1.8 7.6 1.1 10.3-1.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCirclePlus;
impl IconShape for LdMessageCirclePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="M8 12h8" />
  <path d="M12 8v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleQuestion;
impl IconShape for LdMessageCircleQuestion {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
  <path d="M12 17h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleReply;
impl IconShape for LdMessageCircleReply {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="m10 15-3-3 3-3" />
  <path d="M7 12h7a2 2 0 0 1 2 2v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleWarning;
impl IconShape for LdMessageCircleWarning {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="M12 8v4" />
  <path d="M12 16h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircleX;
impl IconShape for LdMessageCircleX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
  <path d="m15 9-6 6" />
  <path d="m9 9 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageCircle;
impl IconShape for LdMessageCircle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareCode;
impl IconShape for LdMessageSquareCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="m10 8-2 2 2 2" />
  <path d="m14 8 2 2-2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareDashed;
impl IconShape for LdMessageSquareDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 6V5c0-1.1.9-2 2-2h2" />
  <path d="M11 3h3" />
  <path d="M18 3h1c1.1 0 2 .9 2 2" />
  <path d="M21 9v2" />
  <path d="M21 15c0 1.1-.9 2-2 2h-1" />
  <path d="M14 17h-3" />
  <path d="m7 17-4 4v-5" />
  <path d="M3 12v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareDiff;
impl IconShape for LdMessageSquareDiff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m5 19-2 2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2" />
  <path d="M9 10h6" />
  <path d="M12 7v6" />
  <path d="M9 17h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareDot;
impl IconShape for LdMessageSquareDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11.7 3H5a2 2 0 0 0-2 2v16l4-4h12a2 2 0 0 0 2-2v-2.7" />
  <circle cx="18" cy="6" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareHeart;
impl IconShape for LdMessageSquareHeart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="M14.8 7.5a1.84 1.84 0 0 0-2.6 0l-.2.3-.3-.3a1.84 1.84 0 1 0-2.4 2.8L12 13l2.7-2.7c.9-.9.8-2.1.1-2.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareMore;
impl IconShape for LdMessageSquareMore {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="M8 10h.01" />
  <path d="M12 10h.01" />
  <path d="M16 10h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareOff;
impl IconShape for LdMessageSquareOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15V5a2 2 0 0 0-2-2H9" />
  <path d="m2 2 20 20" />
  <path d="M3.6 3.6c-.4.3-.6.8-.6 1.4v16l4-4h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquarePlus;
impl IconShape for LdMessageSquarePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="M12 7v6" />
  <path d="M9 10h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareQuote;
impl IconShape for LdMessageSquareQuote {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="M8 12a2 2 0 0 0 2-2V8H8" />
  <path d="M14 12a2 2 0 0 0 2-2V8h-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareReply;
impl IconShape for LdMessageSquareReply {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="m10 7-3 3 3 3" />
  <path d="M17 13v-1a2 2 0 0 0-2-2H7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareShare;
impl IconShape for LdMessageSquareShare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12v3a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h7" />
  <path d="M16 3h5v5" />
  <path d="m16 8 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareText;
impl IconShape for LdMessageSquareText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="M13 8H7" />
  <path d="M17 12H7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareWarning;
impl IconShape for LdMessageSquareWarning {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="M12 7v2" />
  <path d="M12 13h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquareX;
impl IconShape for LdMessageSquareX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
  <path d="m14.5 7.5-5 5" />
  <path d="m9.5 7.5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessageSquare;
impl IconShape for LdMessageSquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMessagesSquare;
impl IconShape for LdMessagesSquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 9a2 2 0 0 1-2 2H6l-4 4V4c0-1.1.9-2 2-2h8a2 2 0 0 1 2 2z" />
  <path d="M18 9h2a2 2 0 0 1 2 2v11l-4-4h-6a2 2 0 0 1-2-2v-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMicOff;
impl IconShape for LdMicOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="22" y1="2" y2="22" />
  <path d="M18.89 13.23A7.12 7.12 0 0 0 19 12v-2" />
  <path d="M5 10v2a7 7 0 0 0 12 5" />
  <path d="M15 9.34V5a3 3 0 0 0-5.68-1.33" />
  <path d="M9 9v3a3 3 0 0 0 5.12 2.12" />
  <line x1="12" x2="12" y1="19" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMicVocal;
impl IconShape for LdMicVocal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 8-9.04 9.06a2.82 2.82 0 1 0 3.98 3.98L16 12" />
  <circle cx="17" cy="7" r="5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMic;
impl IconShape for LdMic {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
  <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
  <line x1="12" x2="12" y1="19" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMicroscope;
impl IconShape for LdMicroscope {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 18h8" />
  <path d="M3 22h18" />
  <path d="M14 22a7 7 0 1 0 0-14h-1" />
  <path d="M9 14h2" />
  <path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z" />
  <path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMicrowave;
impl IconShape for LdMicrowave {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="15" x="2" y="4" rx="2" />
  <rect width="8" height="7" x="6" y="8" rx="1" />
  <path d="M18 8v7" />
  <path d="M6 19v2" />
  <path d="M18 19v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMilestone;
impl IconShape for LdMilestone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 6H5a2 2 0 0 0-2 2v3a2 2 0 0 0 2 2h13l4-3.5L18 6Z" />
  <path d="M12 13v8" />
  <path d="M12 3v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMilkOff;
impl IconShape for LdMilkOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2h8" />
  <path d="M9 2v1.343M15 2v2.789a4 4 0 0 0 .672 2.219l.656.984a4 4 0 0 1 .672 2.22v1.131M7.8 7.8l-.128.192A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-3" />
  <path d="M7 15a6.47 6.47 0 0 1 5 0 6.472 6.472 0 0 0 3.435.435" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMilk;
impl IconShape for LdMilk {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2h8" />
  <path d="M9 2v2.789a4 4 0 0 1-.672 2.219l-.656.984A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-9.789a4 4 0 0 0-.672-2.219l-.656-.984A4 4 0 0 1 15 4.788V2" />
  <path d="M7 15a6.472 6.472 0 0 1 5 0 6.47 6.47 0 0 0 5 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMinimize2;
impl IconShape for LdMinimize2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="4 14 10 14 10 20" />
  <polyline points="20 10 14 10 14 4" />
  <line x1="14" x2="21" y1="10" y2="3" />
  <line x1="3" x2="10" y1="21" y2="14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMinimize;
impl IconShape for LdMinimize {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3v3a2 2 0 0 1-2 2H3" />
  <path d="M21 8h-3a2 2 0 0 1-2-2V3" />
  <path d="M3 16h3a2 2 0 0 1 2 2v3" />
  <path d="M16 21v-3a2 2 0 0 1 2-2h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMinus;
impl IconShape for LdMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 12h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorCheck;
impl IconShape for LdMonitorCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 10 2 2 4-4" />
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorDot;
impl IconShape for LdMonitorDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="19" cy="6" r="3" />
  <path d="M22 12v3a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h9" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorDown;
impl IconShape for LdMonitorDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 13V7" />
  <path d="m15 10-3 3-3-3" />
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorOff;
impl IconShape for LdMonitorOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 17H4a2 2 0 0 1-2-2V5c0-1.5 1-2 1-2" />
  <path d="M22 15V5a2 2 0 0 0-2-2H9" />
  <path d="M8 21h8" />
  <path d="M12 17v4" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorPause;
impl IconShape for LdMonitorPause {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 13V7" />
  <path d="M14 13V7" />
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorPlay;
impl IconShape for LdMonitorPlay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 7.75a.75.75 0 0 1 1.142-.638l3.664 2.249a.75.75 0 0 1 0 1.278l-3.664 2.25a.75.75 0 0 1-1.142-.64z" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
  <rect x="2" y="3" width="20" height="14" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorSmartphone;
impl IconShape for LdMonitorSmartphone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8" />
  <path d="M10 19v-3.96 3.15" />
  <path d="M7 19h5" />
  <rect width="6" height="10" x="16" y="12" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorSpeaker;
impl IconShape for LdMonitorSpeaker {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5.5 20H8" />
  <path d="M17 9h.01" />
  <rect width="10"  x="12" y="4" rx="2" />
  <path d="M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" />
  <circle cx="17" cy="15" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorStop;
impl IconShape for LdMonitorStop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 17v4" />
  <path d="M8 21h8" />
  <rect x="2" y="3" width="20" height="14" rx="2" />
  <rect x="9" y="7" width="6" height="6" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorUp;
impl IconShape for LdMonitorUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 10 3-3 3 3" />
  <path d="M12 13V7" />
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitorX;
impl IconShape for LdMonitorX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m14.5 12.5-5-5" />
  <path d="m9.5 12.5 5-5" />
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <path d="M12 17v4" />
  <path d="M8 21h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMonitor;
impl IconShape for LdMonitor {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="14" x="2" y="3" rx="2" />
  <line x1="8" x2="16" y1="21" y2="21" />
  <line x1="12" x2="12" y1="17" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoonStar;
impl IconShape for LdMoonStar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
  <path d="M19 3v4" />
  <path d="M21 5h-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoon;
impl IconShape for LdMoon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMountainSnow;
impl IconShape for LdMountainSnow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8 3 4 8 5-5 5 15H2L8 3z" />
  <path d="M4.14 15.08c2.62-1.57 5.24-1.43 7.86.42 2.74 1.94 5.49 2 8.23.19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMountain;
impl IconShape for LdMountain {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8 3 4 8 5-5 5 15H2L8 3z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMouseOff;
impl IconShape for LdMouseOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 6v.343" />
  <path d="M18.218 18.218A7 7 0 0 1 5 15V9a7 7 0 0 1 .782-3.218" />
  <path d="M19 13.343V9A7 7 0 0 0 8.56 2.902" />
  <path d="M22 22 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMousePointer2;
impl IconShape for LdMousePointer2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m4 4 7.07 17 2.51-7.39L21 11.07z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMousePointerClick;
impl IconShape for LdMousePointerClick {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 9 5 12 1.8-5.2L21 14Z" />
  <path d="M7.2 2.2 8 5.1" />
  <path d="m5.1 8-2.9-.8" />
  <path d="M14 4.1 12 6" />
  <path d="m6 12-1.9 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMousePointer;
impl IconShape for LdMousePointer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 3 7.07 16.97 2.51-7.39 7.39-2.51L3 3z" />
  <path d="m13 13 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMouse;
impl IconShape for LdMouse {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="5" y="2" width="14" height="20" rx="7" />
  <path d="M12 6v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMove3d;
impl IconShape for LdMove3d {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 3v16h16" />
  <path d="m5 19 6-6" />
  <path d="m2 6 3-3 3 3" />
  <path d="m18 16 3 3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveDiagonal2;
impl IconShape for LdMoveDiagonal2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="5 11 5 5 11 5" />
  <polyline points="19 13 19 19 13 19" />
  <line x1="5" x2="19" y1="5" y2="19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveDiagonal;
impl IconShape for LdMoveDiagonal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="13 5 19 5 19 11" />
  <polyline points="11 19 5 19 5 13" />
  <line x1="19" x2="5" y1="5" y2="19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveDownLeft;
impl IconShape for LdMoveDownLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 19H5V13" />
  <path d="M19 5L5 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveDownRight;
impl IconShape for LdMoveDownRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 13V19H13" />
  <path d="M5 5L19 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveDown;
impl IconShape for LdMoveDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 18L12 22L16 18" />
  <path d="M12 2V22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveHorizontal;
impl IconShape for LdMoveHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="18 8 22 12 18 16" />
  <polyline points="6 8 2 12 6 16" />
  <line x1="2" x2="22" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveLeft;
impl IconShape for LdMoveLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 8L2 12L6 16" />
  <path d="M2 12H22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveRight;
impl IconShape for LdMoveRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 8L22 12L18 16" />
  <path d="M2 12H22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveUpLeft;
impl IconShape for LdMoveUpLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 11V5H11" />
  <path d="M5 5L19 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveUpRight;
impl IconShape for LdMoveUpRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 5H19V11" />
  <path d="M19 5L5 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveUp;
impl IconShape for LdMoveUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 6L12 2L16 6" />
  <path d="M12 2V22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMoveVertical;
impl IconShape for LdMoveVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="8 18 12 22 16 18" />
  <polyline points="8 6 12 2 16 6" />
  <line x1="12" x2="12" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMove;
impl IconShape for LdMove {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="5 9 2 12 5 15" />
  <polyline points="9 5 12 2 15 5" />
  <polyline points="15 19 12 22 9 19" />
  <polyline points="19 9 22 12 19 15" />
  <line x1="2" x2="22" y1="12" y2="12" />
  <line x1="12" x2="12" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMusic2;
impl IconShape for LdMusic2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="8" cy="18" r="4" />
  <path d="M12 18V2l7 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMusic3;
impl IconShape for LdMusic3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="18" r="4" />
  <path d="M16 18V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMusic4;
impl IconShape for LdMusic4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 18V5l12-2v13" />
  <path d="m9 9 12-2" />
  <circle cx="6" cy="18" r="3" />
  <circle cx="18" cy="16" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdMusic;
impl IconShape for LdMusic {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 18V5l12-2v13" />
  <circle cx="6" cy="18" r="3" />
  <circle cx="18" cy="16" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNavigation2Off;
impl IconShape for LdNavigation2Off {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9.31 9.31 5 21l7-4 7 4-1.17-3.17" />
  <path d="M14.53 8.88 12 2l-1.17 3.17" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNavigation2;
impl IconShape for LdNavigation2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="12 2 19 21 12 17 5 21 12 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNavigationOff;
impl IconShape for LdNavigationOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.43 8.43 3 11l8 2 2 8 2.57-5.43" />
  <path d="M17.39 11.73 22 2l-9.73 4.61" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNavigation;
impl IconShape for LdNavigation {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="3 11 22 2 13 21 11 13 3 11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNetwork;
impl IconShape for LdNetwork {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="16" y="16" width="6" height="6" rx="1" />
  <rect x="2" y="16" width="6" height="6" rx="1" />
  <rect x="9" y="2" width="6" height="6" rx="1" />
  <path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3" />
  <path d="M12 12V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNewspaper;
impl IconShape for LdNewspaper {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2" />
  <path d="M18 14h-8" />
  <path d="M15 18h-5" />
  <path d="M10 6h8v4h-8V6Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNfc;
impl IconShape for LdNfc {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 8.32a7.43 7.43 0 0 1 0 7.36" />
  <path d="M9.46 6.21a11.76 11.76 0 0 1 0 11.58" />
  <path d="M12.91 4.1a15.91 15.91 0 0 1 .01 15.8" />
  <path d="M16.37 2a20.16 20.16 0 0 1 0 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNotebookPen;
impl IconShape for LdNotebookPen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.4 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7.4" />
  <path d="M2 6h4" />
  <path d="M2 10h4" />
  <path d="M2 14h4" />
  <path d="M2 18h4" />
  <path d="M18.4 2.6a2.17 2.17 0 0 1 3 3L16 11l-4 1 1-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNotebookTabs;
impl IconShape for LdNotebookTabs {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 6h4" />
  <path d="M2 10h4" />
  <path d="M2 14h4" />
  <path d="M2 18h4" />
  <rect  height="20" x="4" y="2" rx="2" />
  <path d="M15 2v20" />
  <path d="M15 7h5" />
  <path d="M15 12h5" />
  <path d="M15 17h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNotebookText;
impl IconShape for LdNotebookText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 6h4" />
  <path d="M2 10h4" />
  <path d="M2 14h4" />
  <path d="M2 18h4" />
  <rect  height="20" x="4" y="2" rx="2" />
  <path d="M9.5 8h5" />
  <path d="M9.5 12H16" />
  <path d="M9.5 16H14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNotebook;
impl IconShape for LdNotebook {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 6h4" />
  <path d="M2 10h4" />
  <path d="M2 14h4" />
  <path d="M2 18h4" />
  <rect  height="20" x="4" y="2" rx="2" />
  <path d="M16 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNotepadTextDashed;
impl IconShape for LdNotepadTextDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M12 2v4" />
  <path d="M16 2v4" />
  <path d="M16 4h2a2 2 0 0 1 2 2v2" />
  <path d="M20 12v2" />
  <path d="M20 18v2a2 2 0 0 1-2 2h-1" />
  <path d="M13 22h-2" />
  <path d="M7 22H6a2 2 0 0 1-2-2v-2" />
  <path d="M4 14v-2" />
  <path d="M4 8V6a2 2 0 0 1 2-2h2" />
  <path d="M8 10h6" />
  <path d="M8 14h8" />
  <path d="M8 18h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNotepadText;
impl IconShape for LdNotepadText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 2v4" />
  <path d="M12 2v4" />
  <path d="M16 2v4" />
  <rect  height="18" x="4" y="4" rx="2" />
  <path d="M8 10h6" />
  <path d="M8 14h8" />
  <path d="M8 18h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNutOff;
impl IconShape for LdNutOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 4V2" />
  <path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592a7.01 7.01 0 0 0 4.125-2.939" />
  <path d="M19 10v3.343" />
  <path d="M12 12c-1.349-.573-1.905-1.005-2.5-2-.546.902-1.048 1.353-2.5 2-1.018-.644-1.46-1.08-2-2-1.028.71-1.69.918-3 1 1.081-1.048 1.757-2.03 2-3 .194-.776.84-1.551 1.79-2.21m11.654 5.997c.887-.457 1.28-.891 1.556-1.787 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4-.74 0-1.461.068-2.15.192" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdNut;
impl IconShape for LdNut {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 4V2" />
  <path d="M5 10v4a7.004 7.004 0 0 0 5.277 6.787c.412.104.802.292 1.102.592L12 22l.621-.621c.3-.3.69-.488 1.102-.592A7.003 7.003 0 0 0 19 14v-4" />
  <path d="M12 4C8 4 4.5 6 4 8c-.243.97-.919 1.952-2 3 1.31-.082 1.972-.29 3-1 .54.92.982 1.356 2 2 1.452-.647 1.954-1.098 2.5-2 .595.995 1.151 1.427 2.5 2 1.31-.621 1.862-1.058 2.5-2 .629.977 1.162 1.423 2.5 2 1.209-.548 1.68-.967 2-2 1.032.916 1.683 1.157 3 1-1.297-1.036-1.758-2.03-2-3-.5-2-4-4-8-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOctagonAlert;
impl IconShape for LdOctagonAlert {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" />
  <line x1="12" x2="12" y1="8" y2="12" />
  <line x1="12" x2="12.01" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOctagonPause;
impl IconShape for LdOctagonPause {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 15V9" />
  <path d="M14 15V9" />
  <path d="M7.714 2h8.572L22 7.714v8.572L16.286 22H7.714L2 16.286V7.714z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOctagonX;
impl IconShape for LdOctagonX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" />
  <path d="m15 9-6 6" />
  <path d="m9 9 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOctagon;
impl IconShape for LdOctagon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOption;
impl IconShape for LdOption {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3h6l6 18h6" />
  <path d="M14 3h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOrbit;
impl IconShape for LdOrbit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="3" />
  <circle cx="19" cy="5" r="2" />
  <circle cx="5" cy="19" r="2" />
  <path d="M10.4 21.9a10 10 0 0 0 9.941-15.416" />
  <path d="M13.5 2.1a10 10 0 0 0-9.841 15.416" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdOrigami;
impl IconShape for LdOrigami {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 12V4a1 1 0 0 1 1-1h6.297a1 1 0 0 1 .651 1.759l-4.696 4.025" />
  <path d="m12 21-7.414-7.414A2 2 0 0 1 4 12.172V6.415a1.002 1.002 0 0 1 1.707-.707L20 20.009" />
  <path d="m12.214 3.381 8.414 14.966a1 1 0 0 1-.167 1.199l-1.168 1.163a1 1 0 0 1-.706.291H6.351a1 1 0 0 1-.625-.219L3.25 18.8a1 1 0 0 1 .631-1.781l4.165.027" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackage2;
impl IconShape for LdPackage2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z" />
  <path d="m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9" />
  <path d="M12 3v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackageCheck;
impl IconShape for LdPackageCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 16 2 2 4-4" />
  <path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
  <path d="m7.5 4.27 9 5.15" />
  <polyline points="3.29 7 12 12 20.71 7" />
  <line x1="12" x2="12" y1="22" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackageMinus;
impl IconShape for LdPackageMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 16h6" />
  <path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
  <path d="m7.5 4.27 9 5.15" />
  <polyline points="3.29 7 12 12 20.71 7" />
  <line x1="12" x2="12" y1="22" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackageOpen;
impl IconShape for LdPackageOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22v-9" />
  <path d="M15.17 2.21a1.67 1.67 0 0 1 1.63 0L21 4.57a1.93 1.93 0 0 1 0 3.36L8.82 14.79a1.655 1.655 0 0 1-1.64 0L3 12.43a1.93 1.93 0 0 1 0-3.36z" />
  <path d="M20 13v3.87a2.06 2.06 0 0 1-1.11 1.83l-6 3.08a1.93 1.93 0 0 1-1.78 0l-6-3.08A2.06 2.06 0 0 1 4 16.87V13" />
  <path d="M21 12.43a1.93 1.93 0 0 0 0-3.36L8.83 2.2a1.64 1.64 0 0 0-1.63 0L3 4.57a1.93 1.93 0 0 0 0 3.36l12.18 6.86a1.636 1.636 0 0 0 1.63 0z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackagePlus;
impl IconShape for LdPackagePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 16h6" />
  <path d="M19 13v6" />
  <path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
  <path d="m7.5 4.27 9 5.15" />
  <polyline points="3.29 7 12 12 20.71 7" />
  <line x1="12" x2="12" y1="22" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackageSearch;
impl IconShape for LdPackageSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
  <path d="m7.5 4.27 9 5.15" />
  <polyline points="3.29 7 12 12 20.71 7" />
  <line x1="12" x2="12" y1="22" y2="12" />
  <circle cx="18.5" cy="15.5" r="2.5" />
  <path d="M20.27 17.27 22 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackageX;
impl IconShape for LdPackageX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" />
  <path d="m7.5 4.27 9 5.15" />
  <polyline points="3.29 7 12 12 20.71 7" />
  <line x1="12" x2="12" y1="22" y2="12" />
  <path d="m17 13 5 5m-5 0 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPackage;
impl IconShape for LdPackage {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7.5 4.27 9 5.15" />
  <path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" />
  <path d="m3.3 7 8.7 5 8.7-5" />
  <path d="M12 22V12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPaintBucket;
impl IconShape for LdPaintBucket {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 11-8-8-8.6 8.6a2 2 0 0 0 0 2.8l5.2 5.2c.8.8 2 .8 2.8 0L19 11Z" />
  <path d="m5 2 5 5" />
  <path d="M2 13h15" />
  <path d="M22 20a2 2 0 1 1-4 0c0-1.6 1.7-2.4 2-4 .3 1.6 2 2.4 2 4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPaintRoller;
impl IconShape for LdPaintRoller {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="6" x="2" y="2" rx="2" />
  <path d="M10 16v-2a2 2 0 0 1 2-2h8a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" />
  <rect width="4" height="6" x="8" y="16" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPaintbrush2;
impl IconShape for LdPaintbrush2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 19.9V16h3a2 2 0 0 0 2-2v-2H5v2c0 1.1.9 2 2 2h3v3.9a2 2 0 1 0 4 0Z" />
  <path d="M6 12V2h12v10" />
  <path d="M14 2v4" />
  <path d="M10 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPaintbrush;
impl IconShape for LdPaintbrush {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.37 2.63 14 7l-1.59-1.59a2 2 0 0 0-2.82 0L8 7l9 9 1.59-1.59a2 2 0 0 0 0-2.82L17 10l4.37-4.37a2.12 2.12 0 1 0-3-3Z" />
  <path d="M9 8c-2 3-4 3.5-7 4l8 10c2-1 6-5 6-7" />
  <path d="M14.5 17.5 4.5 15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPalette;
impl IconShape for LdPalette {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="13.5" cy="6.5" r=".5" fill="currentColor" />
  <circle cx="17.5" cy="10.5" r=".5" fill="currentColor" />
  <circle cx="8.5" cy="7.5" r=".5" fill="currentColor" />
  <circle cx="6.5" cy="12.5" r=".5" fill="currentColor" />
  <path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelBottomClose;
impl IconShape for LdPanelBottomClose {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 15h18" />
  <path d="m15 8-3 3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelBottomDashed;
impl IconShape for LdPanelBottomDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M14 15h1" />
  <path d="M19 15h2" />
  <path d="M3 15h2" />
  <path d="M9 15h1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelBottomOpen;
impl IconShape for LdPanelBottomOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 15h18" />
  <path d="m9 10 3-3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelBottom;
impl IconShape for LdPanelBottom {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 15h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelLeftClose;
impl IconShape for LdPanelLeftClose {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 3v18" />
  <path d="m16 15-3-3 3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelLeftDashed;
impl IconShape for LdPanelLeftDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 14v1" />
  <path d="M9 19v2" />
  <path d="M9 3v2" />
  <path d="M9 9v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelLeftOpen;
impl IconShape for LdPanelLeftOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 3v18" />
  <path d="m14 9 3 3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelLeft;
impl IconShape for LdPanelLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelRightClose;
impl IconShape for LdPanelRightClose {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M15 3v18" />
  <path d="m8 9 3 3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelRightDashed;
impl IconShape for LdPanelRightDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M15 14v1" />
  <path d="M15 19v2" />
  <path d="M15 3v2" />
  <path d="M15 9v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelRightOpen;
impl IconShape for LdPanelRightOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M15 3v18" />
  <path d="m10 15-3-3 3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelRight;
impl IconShape for LdPanelRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M15 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelTopClose;
impl IconShape for LdPanelTopClose {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9h18" />
  <path d="m9 16 3-3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelTopDashed;
impl IconShape for LdPanelTopDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M14 9h1" />
  <path d="M19 9h2" />
  <path d="M3 9h2" />
  <path d="M9 9h1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelTopOpen;
impl IconShape for LdPanelTopOpen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9h18" />
  <path d="m15 14-3 3-3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelTop;
impl IconShape for LdPanelTop {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelsLeftBottom;
impl IconShape for LdPanelsLeftBottom {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 3v18" />
  <path d="M9 15h12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelsRightBottom;
impl IconShape for LdPanelsRightBottom {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 15h12" />
  <path d="M15 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPanelsTopLeft;
impl IconShape for LdPanelsTopLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9h18" />
  <path d="M9 21V9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPaperclip;
impl IconShape for LdPaperclip {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdParentheses;
impl IconShape for LdParentheses {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 21s-4-3-4-9 4-9 4-9" />
  <path d="M16 3s4 3 4 9-4 9-4 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdParkingMeter;
impl IconShape for LdParkingMeter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 9a3 3 0 1 1 6 0" />
  <path d="M12 12v3" />
  <path d="M11 15h2" />
  <path d="M19 9a7 7 0 1 0-13.6 2.3C6.4 14.4 8 19 8 19h8s1.6-4.6 2.6-7.7c.3-.8.4-1.5.4-2.3" />
  <path d="M12 19v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPartyPopper;
impl IconShape for LdPartyPopper {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5.8 11.3 2 22l10.7-3.79" />
  <path d="M4 3h.01" />
  <path d="M22 8h.01" />
  <path d="M15 2h.01" />
  <path d="M22 20h.01" />
  <path d="m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12v0c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10" />
  <path d="m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11v0c-.11.7-.72 1.22-1.43 1.22H17" />
  <path d="m11 2 .33.82c.34.86-.2 1.82-1.11 1.98v0C9.52 4.9 9 5.52 9 6.23V7" />
  <path d="M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPause;
impl IconShape for LdPause {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="14" y="4" width="4"  rx="1" />
  <rect x="6" y="4" width="4"  rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPawPrint;
impl IconShape for LdPawPrint {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11" cy="4" r="2" />
  <circle cx="18" cy="8" r="2" />
  <circle cx="20" cy="16" r="2" />
  <path d="M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPcCase;
impl IconShape for LdPcCase {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="20" x="5" y="2" rx="2" />
  <path d="M15 14h.01" />
  <path d="M9 6h6" />
  <path d="M9 10h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPenLine;
impl IconShape for LdPenLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 20h9" />
  <path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPenTool;
impl IconShape for LdPenTool {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.707 21.293a1 1 0 0 1-1.414 0l-1.586-1.586a1 1 0 0 1 0-1.414l5.586-5.586a1 1 0 0 1 1.414 0l1.586 1.586a1 1 0 0 1 0 1.414z" />
  <path d="m18 13-1.375-6.874a1 1 0 0 0-.746-.776L3.235 2.028a1 1 0 0 0-1.207 1.207L5.35 15.879a1 1 0 0 0 .776.746L13 18" />
  <path d="m2.3 2.3 7.286 7.286" />
  <circle cx="11" cy="11" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPen;
impl IconShape for LdPen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPencilLine;
impl IconShape for LdPencilLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 20h9" />
  <path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" />
  <path d="m15 5 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPencilRuler;
impl IconShape for LdPencilRuler {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 5 4 4" />
  <path d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13" />
  <path d="m8 6 2-2" />
  <path d="m2 22 5.5-1.5L21.17 6.83a2.82 2.82 0 0 0-4-4L3.5 16.5Z" />
  <path d="m18 16 2-2" />
  <path d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPencil;
impl IconShape for LdPencil {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
  <path d="m15 5 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPentagon;
impl IconShape for LdPentagon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.5 8.7c-.7.5-1 1.4-.7 2.2l2.8 8.7c.3.8 1 1.4 1.9 1.4h9.1c.9 0 1.6-.6 1.9-1.4l2.8-8.7c.3-.8 0-1.7-.7-2.2l-7.4-5.3a2.1 2.1 0 0 0-2.4 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPercent;
impl IconShape for LdPercent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="19" x2="5" y1="5" y2="19" />
  <circle cx="6.5" cy="6.5" r="2.5" />
  <circle cx="17.5" cy="17.5" r="2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPersonStanding;
impl IconShape for LdPersonStanding {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="5" r="1" />
  <path d="m9 20 3-6 3 6" />
  <path d="m6 8 6 2 6-2" />
  <path d="M12 10v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhoneCall;
impl IconShape for LdPhoneCall {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
  <path d="M14.05 2a9 9 0 0 1 8 7.94" />
  <path d="M14.05 6A5 5 0 0 1 18 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhoneForwarded;
impl IconShape for LdPhoneForwarded {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="18 2 22 6 18 10" />
  <line x1="14" x2="22" y1="6" y2="6" />
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhoneIncoming;
impl IconShape for LdPhoneIncoming {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="16 2 16 8 22 8" />
  <line x1="22" x2="16" y1="2" y2="8" />
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhoneMissed;
impl IconShape for LdPhoneMissed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="22" x2="16" y1="2" y2="8" />
  <line x1="16" x2="22" y1="2" y2="8" />
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhoneOff;
impl IconShape for LdPhoneOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91" />
  <line x1="22" x2="2" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhoneOutgoing;
impl IconShape for LdPhoneOutgoing {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="22 8 22 2 16 2" />
  <line x1="16" x2="22" y1="8" y2="2" />
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPhone;
impl IconShape for LdPhone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPi;
impl IconShape for LdPi {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="9" x2="9" y1="4" y2="20" />
  <path d="M4 7c0-1.7 1.3-3 3-3h13" />
  <path d="M18 20c-1.7 0-3-1.3-3-3V4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPiano;
impl IconShape for LdPiano {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.5 8c-1.4 0-2.6-.8-3.2-2A6.87 6.87 0 0 0 2 9v11a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-8.5C22 9.6 20.4 8 18.5 8" />
  <path d="M2 14h20" />
  <path d="M6 14v4" />
  <path d="M10 14v4" />
  <path d="M14 14v4" />
  <path d="M18 14v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPickaxe;
impl IconShape for LdPickaxe {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.531 12.469 6.619 20.38a1 1 0 1 1-3-3l7.912-7.912" />
  <path d="M15.686 4.314A12.5 12.5 0 0 0 5.461 2.958 1 1 0 0 0 5.58 4.71a22 22 0 0 1 6.318 3.393" />
  <path d="M17.7 3.7a1 1 0 0 0-1.4 0l-4.6 4.6a1 1 0 0 0 0 1.4l2.6 2.6a1 1 0 0 0 1.4 0l4.6-4.6a1 1 0 0 0 0-1.4z" />
  <path d="M19.686 8.314a12.501 12.501 0 0 1 1.356 10.225 1 1 0 0 1-1.751-.119 22 22 0 0 0-3.393-6.319" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPictureInPicture2;
impl IconShape for LdPictureInPicture2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4" />
  <rect width="10" height="7" x="12" y="13" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPictureInPicture;
impl IconShape for LdPictureInPicture {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 4.5v5H3m-1-6 6 6m13 0v-3c0-1.16-.84-2-2-2h-7m-9 9v2c0 1.05.95 2 2 2h3" />
  <rect width="10" height="7" x="12" y="13.5" ry="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPieChart;
impl IconShape for LdPieChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.21 15.89A10 10 0 1 1 8 2.83" />
  <path d="M22 12A10 10 0 0 0 12 2v10z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPiggyBank;
impl IconShape for LdPiggyBank {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 5c-1.5 0-2.8 1.4-3 2-3.5-1.5-11-.3-11 5 0 1.8 0 3 2 4.5V20h4v-2h3v2h4v-4c1-.5 1.7-1 2-2h2v-4h-2c0-1-.5-1.5-1-2h0V5z" />
  <path d="M2 9v1c0 1.1.9 2 2 2h1" />
  <path d="M16 11h0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPilcrowLeft;
impl IconShape for LdPilcrowLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 3v11" />
  <path d="M14 9h-3a3 3 0 0 1 0-6h9" />
  <path d="M18 3v11" />
  <path d="M22 18H2l4-4" />
  <path d="m6 22-4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPilcrowRight;
impl IconShape for LdPilcrowRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 3v11" />
  <path d="M10 9H7a1 1 0 0 1 0-6h8" />
  <path d="M14 3v11" />
  <path d="m18 14 4 4H2" />
  <path d="m22 18-4 4" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPilcrow;
impl IconShape for LdPilcrow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 4v16" />
  <path d="M17 4v16" />
  <path d="M19 4H9.5a4.5 4.5 0 0 0 0 9H13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPill;
impl IconShape for LdPill {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z" />
  <path d="m8.5 8.5 7 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPinOff;
impl IconShape for LdPinOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="22" y1="2" y2="22" />
  <line x1="12" x2="12" y1="17" y2="22" />
  <path d="M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12" />
  <path d="M15 9.34V6h1a2 2 0 0 0 0-4H7.89" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPin;
impl IconShape for LdPin {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="12" y1="17" y2="22" />
  <path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPipette;
impl IconShape for LdPipette {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 22 1-1h3l9-9" />
  <path d="M3 21v-3l9-9" />
  <path d="m15 6 3.4-3.4a2.1 2.1 0 1 1 3 3L18 9l.4.4a2.1 2.1 0 1 1-3 3l-3.8-3.8a2.1 2.1 0 1 1 3-3l.4.4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPizza;
impl IconShape for LdPizza {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 11h.01" />
  <path d="M11 15h.01" />
  <path d="M16 16h.01" />
  <path d="m2 16 20 6-6-20A20 20 0 0 0 2 16" />
  <path d="M5.71 17.11a17.04 17.04 0 0 1 11.4-11.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlaneLanding;
impl IconShape for LdPlaneLanding {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 22h20" />
  <path d="M3.77 10.77 2 9l2-4.5 1.1.55c.55.28.9.84.9 1.45s.35 1.17.9 1.45L8 8.5l3-6 1.05.53a2 2 0 0 1 1.09 1.52l.72 5.4a2 2 0 0 0 1.09 1.52l4.4 2.2c.42.22.78.55 1.01.96l.6 1.03c.49.88-.06 1.98-1.06 2.1l-1.18.15c-.47.06-.95-.02-1.37-.24L4.29 11.15a2 2 0 0 1-.52-.38Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlaneTakeoff;
impl IconShape for LdPlaneTakeoff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 22h20" />
  <path d="M6.36 17.4 4 17l-2-4 1.1-.55a2 2 0 0 1 1.8 0l.17.1a2 2 0 0 0 1.8 0L8 12 5 6l.9-.45a2 2 0 0 1 2.09.2l4.02 3a2 2 0 0 0 2.1.2l4.19-2.06a2.41 2.41 0 0 1 1.73-.17L21 7a1.4 1.4 0 0 1 .87 1.99l-.38.76c-.23.46-.6.84-1.07 1.08L7.58 17.2a2 2 0 0 1-1.22.18Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlane;
impl IconShape for LdPlane {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3c-1-.5-3 0-4.5 1.5L13 8 4.8 6.2c-.5-.1-.9.1-1.1.5l-.3.5c-.2.5-.1 1 .3 1.3L9 12l-2 3H4l-1 1 3 2 2 3 1-1v-3l3-2 3.5 5.3c.3.4.8.5 1.3.3l.5-.2c.4-.3.6-.7.5-1.2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlay;
impl IconShape for LdPlay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="6 3 20 12 6 21 6 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlug2;
impl IconShape for LdPlug2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 2v6" />
  <path d="M15 2v6" />
  <path d="M12 17v5" />
  <path d="M5 8h14" />
  <path d="M6 11V8h12v3a6 6 0 1 1-12 0v0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlugZap2;
impl IconShape for LdPlugZap2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m13 2-2 2.5h3L12 7" />
  <path d="M10 14v-3" />
  <path d="M14 14v-3" />
  <path d="M11 19c-1.7 0-3-1.3-3-3v-2h8v2c0 1.7-1.3 3-3 3Z" />
  <path d="M12 22v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlugZap;
impl IconShape for LdPlugZap {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z" />
  <path d="m2 22 3-3" />
  <path d="M7.5 13.5 10 11" />
  <path d="M10.5 16.5 13 14" />
  <path d="m18 3-4 4h6l-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlug;
impl IconShape for LdPlug {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22v-5" />
  <path d="M9 8V2" />
  <path d="M15 8V2" />
  <path d="M18 8v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V8Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPlus;
impl IconShape for LdPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 12h14" />
  <path d="M12 5v14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPocketKnife;
impl IconShape for LdPocketKnife {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 2v1c0 1 2 1 2 2S3 6 3 7s2 1 2 2-2 1-2 2 2 1 2 2" />
  <path d="M18 6h.01" />
  <path d="M6 18h.01" />
  <path d="M20.83 8.83a4 4 0 0 0-5.66-5.66l-12 12a4 4 0 1 0 5.66 5.66Z" />
  <path d="M18 11.66V22a4 4 0 0 0 4-4V6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPocket;
impl IconShape for LdPocket {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z" />
  <polyline points="8 10 12 14 16 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPodcast;
impl IconShape for LdPodcast {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16.85 18.58a9 9 0 1 0-9.7 0" />
  <path d="M8 14a5 5 0 1 1 8 0" />
  <circle cx="12" cy="11" r="1" />
  <path d="M13 17a1 1 0 1 0-2 0l.5 4.5a.5.5 0 1 0 1 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPointerOff;
impl IconShape for LdPointerOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 4.5V4a2 2 0 0 0-2.41-1.957" />
  <path d="M13.9 8.4a2 2 0 0 0-1.26-1.295" />
  <path d="M21.7 16.2A8 8 0 0 0 22 14v-3a2 2 0 1 0-4 0v-1a2 2 0 0 0-3.63-1.158" />
  <path d="m7 15-1.8-1.8a2 2 0 0 0-2.79 2.86L6 19.7a7.74 7.74 0 0 0 6 2.3h2a8 8 0 0 0 5.657-2.343" />
  <path d="M6 6v8" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPointer;
impl IconShape for LdPointer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 14a8 8 0 0 1-8 8" />
  <path d="M18 11v-1a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" />
  <path d="M14 10V9a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v1" />
  <path d="M10 9.5V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v10" />
  <path d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPopcorn;
impl IconShape for LdPopcorn {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 8a2 2 0 0 0 0-4 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0 0 4" />
  <path d="M10 22 9 8" />
  <path d="m14 22 1-14" />
  <path d="M20 8c.5 0 .9.4.8 1l-2.6 12c-.1.5-.7 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L3.2 9c-.1-.6.3-1 .8-1Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPopsicle;
impl IconShape for LdPopsicle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.6 14.4c.8-.8.8-2 0-2.8l-8.1-8.1a4.95 4.95 0 1 0-7.1 7.1l8.1 8.1c.9.7 2.1.7 2.9-.1Z" />
  <path d="m22 22-5.5-5.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPoundSterling;
impl IconShape for LdPoundSterling {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 7c0-5.333-8-5.333-8 0" />
  <path d="M10 7v14" />
  <path d="M6 21h12" />
  <path d="M6 13h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPowerOff;
impl IconShape for LdPowerOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.36 6.64A9 9 0 0 1 20.77 15" />
  <path d="M6.16 6.16a9 9 0 1 0 12.68 12.68" />
  <path d="M12 2v4" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPower;
impl IconShape for LdPower {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v10" />
  <path d="M18.4 6.6a9 9 0 1 1-12.77.04" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPresentation;
impl IconShape for LdPresentation {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 3h20" />
  <path d="M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3" />
  <path d="m7 21 5-5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPrinter;
impl IconShape for LdPrinter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="6 9 6 2 18 2 18 9" />
  <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
  <rect width="12" height="8" x="6" y="14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdProjector;
impl IconShape for LdProjector {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 7 3 5" />
  <path d="M9 6V3" />
  <path d="m13 7 2-2" />
  <circle cx="9" cy="13" r="3" />
  <path d="M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17" />
  <path d="M16 16h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdProportions;
impl IconShape for LdProportions {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="M12 9v11" />
  <path d="M2 9h13a2 2 0 0 1 2 2v9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPuzzle;
impl IconShape for LdPuzzle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19.439 7.85c-.049.322.059.648.289.878l1.568 1.568c.47.47.706 1.087.706 1.704s-.235 1.233-.706 1.704l-1.611 1.611a.98.98 0 0 1-.837.276c-.47-.07-.802-.48-.968-.925a2.501 2.501 0 1 0-3.214 3.214c.446.166.855.497.925.968a.979.979 0 0 1-.276.837l-1.61 1.61a2.404 2.404 0 0 1-1.705.707 2.402 2.402 0 0 1-1.704-.706l-1.568-1.568a1.026 1.026 0 0 0-.877-.29c-.493.074-.84.504-1.02.968a2.5 2.5 0 1 1-3.237-3.237c.464-.18.894-.527.967-1.02a1.026 1.026 0 0 0-.289-.877l-1.568-1.568A2.402 2.402 0 0 1 1.998 12c0-.617.236-1.234.706-1.704L4.23 8.77c.24-.24.581-.353.917-.303.515.077.877.528 1.073 1.01a2.5 2.5 0 1 0 3.259-3.259c-.482-.196-.933-.558-1.01-1.073-.05-.336.062-.676.303-.917l1.525-1.525A2.402 2.402 0 0 1 12 1.998c.617 0 1.234.236 1.704.706l1.568 1.568c.23.23.556.338.877.29.493-.074.84-.504 1.02-.968a2.5 2.5 0 1 1 3.237 3.237c-.464.18-.894.527-.967 1.02Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdPyramid;
impl IconShape for LdPyramid {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.5 16.88a1 1 0 0 1-.32-1.43l9-13.02a1 1 0 0 1 1.64 0l9 13.01a1 1 0 0 1-.32 1.44l-8.51 4.86a2 2 0 0 1-1.98 0Z" />
  <path d="M12 2v20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdQrCode;
impl IconShape for LdQrCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="5" height="5" x="3" y="3" rx="1" />
  <rect width="5" height="5" x="16" y="3" rx="1" />
  <rect width="5" height="5" x="3" y="16" rx="1" />
  <path d="M21 16h-3a2 2 0 0 0-2 2v3" />
  <path d="M21 21v.01" />
  <path d="M12 7v3a2 2 0 0 1-2 2H7" />
  <path d="M3 12h.01" />
  <path d="M12 3h.01" />
  <path d="M12 16v.01" />
  <path d="M16 12h1" />
  <path d="M21 12v.01" />
  <path d="M12 21v-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdQuote;
impl IconShape for LdQuote {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z" />
  <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRabbit;
impl IconShape for LdRabbit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 16a3 3 0 0 1 2.24 5" />
  <path d="M18 12h.01" />
  <path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1 1 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1a3 3 0 0 0-3 3" />
  <path d="M20 8.54V4a2 2 0 1 0-4 0v3" />
  <path d="M7.612 12.524a3 3 0 1 0-1.6 4.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadar;
impl IconShape for LdRadar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19.07 4.93A10 10 0 0 0 6.99 3.34" />
  <path d="M4 6h.01" />
  <path d="M2.29 9.62A10 10 0 1 0 21.31 8.35" />
  <path d="M16.24 7.76A6 6 0 1 0 8.23 16.67" />
  <path d="M12 18h.01" />
  <path d="M17.99 11.66A6 6 0 0 1 15.77 16.67" />
  <circle cx="12" cy="12" r="2" />
  <path d="m13.41 10.59 5.66-5.66" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadiation;
impl IconShape for LdRadiation {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 12h0.01" />
  <path d="M7.5 4.2c-.3-.5-.9-.7-1.3-.4C3.9 5.5 2.3 8.1 2 11c-.1.5.4 1 1 1h5c0-1.5.8-2.8 2-3.4-1.1-1.9-2-3.5-2.5-4.4z" />
  <path d="M21 12c.6 0 1-.4 1-1-.3-2.9-1.8-5.5-4.1-7.1-.4-.3-1.1-.2-1.3.3-.6.9-1.5 2.5-2.6 4.3 1.2.7 2 2 2 3.5h5z" />
  <path d="M7.5 19.8c-.3.5-.1 1.1.4 1.3 2.6 1.2 5.6 1.2 8.2 0 .5-.2.7-.8.4-1.3-.5-.9-1.4-2.5-2.5-4.3-1.2.7-2.8.7-4 0-1.1 1.8-2 3.4-2.5 4.3z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadical;
impl IconShape for LdRadical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 12h4l3 9 4-17h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadioReceiver;
impl IconShape for LdRadioReceiver {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 16v2" />
  <path d="M19 16v2" />
  <rect width="20" height="8" x="2" y="8" rx="2" />
  <path d="M18 12h0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadioTower;
impl IconShape for LdRadioTower {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9" />
  <path d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5" />
  <circle cx="12" cy="9" r="2" />
  <path d="M16.2 4.8c2 2 2.26 5.11.8 7.47" />
  <path d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1" />
  <path d="M9.5 18h5" />
  <path d="m8 22 4-11 4 11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadio;
impl IconShape for LdRadio {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4.9 19.1C1 15.2 1 8.8 4.9 4.9" />
  <path d="M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" />
  <circle cx="12" cy="12" r="2" />
  <path d="M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" />
  <path d="M19.1 4.9C23 8.8 23 15.1 19.1 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRadius;
impl IconShape for LdRadius {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20.34 17.52a10 10 0 1 0-2.82 2.82" />
  <circle cx="19" cy="19" r="2" />
  <path d="m13.41 13.41 4.18 4.18" />
  <circle cx="12" cy="12" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRailSymbol;
impl IconShape for LdRailSymbol {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 15h14" />
  <path d="M5 9h14" />
  <path d="m14 20-5-5 6-6-5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRainbow;
impl IconShape for LdRainbow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 17a10 10 0 0 0-20 0" />
  <path d="M6 17a6 6 0 0 1 12 0" />
  <path d="M10 17a2 2 0 0 1 4 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRat;
impl IconShape for LdRat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 5c0-1.7-1.3-3-3-3s-3 1.3-3 3c0 .8.3 1.5.8 2H11c-3.9 0-7 3.1-7 7v0c0 2.2 1.8 4 4 4" />
  <path d="M16.8 3.9c.3-.3.6-.5 1-.7 1.5-.6 3.3.1 3.9 1.6.6 1.5-.1 3.3-1.6 3.9l1.6 2.8c.2.3.2.7.2 1-.2.8-.9 1.2-1.7 1.1 0 0-1.6-.3-2.7-.6H17c-1.7 0-3 1.3-3 3" />
  <path d="M13.2 18a3 3 0 0 0-2.2-5" />
  <path d="M13 22H4a2 2 0 0 1 0-4h12" />
  <path d="M16 9h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRatio;
impl IconShape for LdRatio {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="12" height="20" x="6" y="2" rx="2" />
  <rect width="20" height="12" x="2" y="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptCent;
impl IconShape for LdReceiptCent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M12 6.5v11" />
  <path d="M15 9.4a4 4 0 1 0 0 5.2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptEuro;
impl IconShape for LdReceiptEuro {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M8 12h5" />
  <path d="M16 9.5a4 4 0 1 0 0 5.2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptIndianRupee;
impl IconShape for LdReceiptIndianRupee {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M8 7h8" />
  <path d="M12 17.5 8 15h1a4 4 0 0 0 0-8" />
  <path d="M8 11h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptJapaneseYen;
impl IconShape for LdReceiptJapaneseYen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="m12 10 3-3" />
  <path d="m9 7 3 3v7.5" />
  <path d="M9 11h6" />
  <path d="M9 15h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptPoundSterling;
impl IconShape for LdReceiptPoundSterling {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M8 13h5" />
  <path d="M10 17V9.5a2.5 2.5 0 0 1 5 0" />
  <path d="M8 17h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptRussianRuble;
impl IconShape for LdReceiptRussianRuble {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M8 15h5" />
  <path d="M8 11h5a2 2 0 1 0 0-4h-3v10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptSwissFranc;
impl IconShape for LdReceiptSwissFranc {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M10 17V7h5" />
  <path d="M10 11h4" />
  <path d="M8 15h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceiptText;
impl IconShape for LdReceiptText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M14 8H8" />
  <path d="M16 12H8" />
  <path d="M13 16H8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReceipt;
impl IconShape for LdReceipt {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z" />
  <path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" />
  <path d="M12 17.5v-11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRectangleEllipsis;
impl IconShape for LdRectangleEllipsis {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="12" x="2" y="6" rx="2" />
  <path d="M12 12h.01" />
  <path d="M17 12h.01" />
  <path d="M7 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRectangleHorizontal;
impl IconShape for LdRectangleHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="12" x="2" y="6" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRectangleVertical;
impl IconShape for LdRectangleVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="12" height="20" x="6" y="2" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRecycle;
impl IconShape for LdRecycle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 19H4.815a1.83 1.83 0 0 1-1.57-.881 1.785 1.785 0 0 1-.004-1.784L7.196 9.5" />
  <path d="M11 19h8.203a1.83 1.83 0 0 0 1.556-.89 1.784 1.784 0 0 0 0-1.775l-1.226-2.12" />
  <path d="m14 16-3 3 3 3" />
  <path d="M8.293 13.596 7.196 9.5 3.1 10.598" />
  <path d="m9.344 5.811 1.093-1.892A1.83 1.83 0 0 1 11.985 3a1.784 1.784 0 0 1 1.546.888l3.943 6.843" />
  <path d="m13.378 9.633 4.096 1.098 1.097-4.096" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRedo2;
impl IconShape for LdRedo2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 14 5-5-5-5" />
  <path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5v0A5.5 5.5 0 0 0 9.5 20H13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRedoDot;
impl IconShape for LdRedoDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="17" r="1" />
  <path d="M21 7v6h-6" />
  <path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRedo;
impl IconShape for LdRedo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 7v6h-6" />
  <path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRefreshCcwDot;
impl IconShape for LdRefreshCcwDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 2v6h6" />
  <path d="M21 12A9 9 0 0 0 6 5.3L3 8" />
  <path d="M21 22v-6h-6" />
  <path d="M3 12a9 9 0 0 0 15 6.7l3-2.7" />
  <circle cx="12" cy="12" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRefreshCcw;
impl IconShape for LdRefreshCcw {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
  <path d="M3 3v5h5" />
  <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
  <path d="M16 16h5v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRefreshCwOff;
impl IconShape for LdRefreshCwOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 8L18.74 5.74A9.75 9.75 0 0 0 12 3C11 3 10.03 3.16 9.13 3.47" />
  <path d="M8 16H3v5" />
  <path d="M3 12C3 9.51 4 7.26 5.64 5.64" />
  <path d="m3 16 2.26 2.26A9.75 9.75 0 0 0 12 21c2.49 0 4.74-1 6.36-2.64" />
  <path d="M21 12c0 1-.16 1.97-.47 2.87" />
  <path d="M21 3v5h-5" />
  <path d="M22 22 2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRefreshCw;
impl IconShape for LdRefreshCw {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
  <path d="M21 3v5h-5" />
  <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
  <path d="M8 16H3v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRefrigerator;
impl IconShape for LdRefrigerator {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 6a4 4 0 0 1 4-4h6a4 4 0 0 1 4 4v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6Z" />
  <path d="M5 10h14" />
  <path d="M15 7v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRegex;
impl IconShape for LdRegex {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 3v10" />
  <path d="m12.67 5.5 8.66 5" />
  <path d="m12.67 10.5 8.66-5" />
  <path d="M9 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRemoveFormatting;
impl IconShape for LdRemoveFormatting {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 7V4h16v3" />
  <path d="M5 20h6" />
  <path d="M13 4 8 20" />
  <path d="m15 15 5 5" />
  <path d="m20 15-5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRepeat1;
impl IconShape for LdRepeat1 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 2 4 4-4 4" />
  <path d="M3 11v-1a4 4 0 0 1 4-4h14" />
  <path d="m7 22-4-4 4-4" />
  <path d="M21 13v1a4 4 0 0 1-4 4H3" />
  <path d="M11 10h1v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRepeat2;
impl IconShape for LdRepeat2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 9 3-3 3 3" />
  <path d="M13 18H7a2 2 0 0 1-2-2V6" />
  <path d="m22 15-3 3-3-3" />
  <path d="M11 6h6a2 2 0 0 1 2 2v10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRepeat;
impl IconShape for LdRepeat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 2 4 4-4 4" />
  <path d="M3 11v-1a4 4 0 0 1 4-4h14" />
  <path d="m7 22-4-4 4-4" />
  <path d="M21 13v1a4 4 0 0 1-4 4H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReplaceAll;
impl IconShape for LdReplaceAll {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 4c0-1.1.9-2 2-2" />
  <path d="M20 2c1.1 0 2 .9 2 2" />
  <path d="M22 8c0 1.1-.9 2-2 2" />
  <path d="M16 10c-1.1 0-2-.9-2-2" />
  <path d="m3 7 3 3 3-3" />
  <path d="M6 10V5c0-1.7 1.3-3 3-3h1" />
  <rect width="8" height="8" x="2" y="14" rx="2" />
  <path d="M14 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
  <path d="M20 14c1.1 0 2 .9 2 2v4c0 1.1-.9 2-2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReplace;
impl IconShape for LdReplace {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 4c0-1.1.9-2 2-2" />
  <path d="M20 2c1.1 0 2 .9 2 2" />
  <path d="M22 8c0 1.1-.9 2-2 2" />
  <path d="M16 10c-1.1 0-2-.9-2-2" />
  <path d="m3 7 3 3 3-3" />
  <path d="M6 10V5c0-1.7 1.3-3 3-3h1" />
  <rect width="8" height="8" x="2" y="14" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReplyAll;
impl IconShape for LdReplyAll {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="7 17 2 12 7 7" />
  <polyline points="12 17 7 12 12 7" />
  <path d="M22 18v-2a4 4 0 0 0-4-4H7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdReply;
impl IconShape for LdReply {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="9 17 4 12 9 7" />
  <path d="M20 18v-2a4 4 0 0 0-4-4H4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRewind;
impl IconShape for LdRewind {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="11 19 2 12 11 5 11 19" />
  <polygon points="22 19 13 12 22 5 22 19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRibbon;
impl IconShape for LdRibbon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.75 9.01c-.52 2.08-1.83 3.64-3.18 5.49l-2.6 3.54-2.97 4-3.5-2.54 3.85-4.97c-1.86-2.61-2.8-3.77-3.16-5.44" />
  <path d="M17.75 9.01A7 7 0 0 0 6.2 9.1C6.06 8.5 6 7.82 6 7c0-3.5 2.83-5 5.98-5C15.24 2 18 3.5 18 7c0 .73-.09 1.4-.25 2.01Z" />
  <path d="m9.35 14.53 2.64-3.31" />
  <path d="m11.97 18.04 2.99 4 3.54-2.54-3.93-5" />
  <path d="M14 8c0 1-1 2-2.01 3.22C11 10 10 9 10 8a2 2 0 1 1 4 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRocket;
impl IconShape for LdRocket {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z" />
  <path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z" />
  <path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" />
  <path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRockingChair;
impl IconShape for LdRockingChair {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="3.5 2 6.5 12.5 18 12.5" />
  <line x1="9.5" x2="5.5" y1="12.5" y2="20" />
  <line x1="15" x2="18.5" y1="12.5" y2="20" />
  <path d="M2.75 18a13 13 0 0 0 18.5 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRollerCoaster;
impl IconShape for LdRollerCoaster {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 19V5" />
  <path d="M10 19V6.8" />
  <path d="M14 19v-7.8" />
  <path d="M18 5v4" />
  <path d="M18 19v-6" />
  <path d="M22 19V9" />
  <path d="M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRotate3d;
impl IconShape for LdRotate3d {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16.466 7.5C15.643 4.237 13.952 2 12 2 9.239 2 7 6.477 7 12s2.239 10 5 10c.342 0 .677-.069 1-.2" />
  <path d="m15.194 13.707 3.814 1.86-1.86 3.814" />
  <path d="M19 15.57c-1.804.885-4.274 1.43-7 1.43-5.523 0-10-2.239-10-5s4.477-5 10-5c4.838 0 8.873 1.718 9.8 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRotateCcwSquare;
impl IconShape for LdRotateCcwSquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 9V7a2 2 0 0 0-2-2h-6" />
  <path d="m15 2-3 3 3 3" />
  <path d="M20 13v5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRotateCcw;
impl IconShape for LdRotateCcw {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
  <path d="M3 3v5h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRotateCwSquare;
impl IconShape for LdRotateCwSquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 5H6a2 2 0 0 0-2 2v3" />
  <path d="m9 8 3-3-3-3" />
  <path d="M4 14v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRotateCw;
impl IconShape for LdRotateCw {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
  <path d="M21 3v5h-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRouteOff;
impl IconShape for LdRouteOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="19" r="3" />
  <path d="M9 19h8.5c.4 0 .9-.1 1.3-.2" />
  <path d="M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12" />
  <path d="m2 2 20 20" />
  <path d="M21 15.3a3.5 3.5 0 0 0-3.3-3.3" />
  <path d="M15 5h-4.3" />
  <circle cx="18" cy="5" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRoute;
impl IconShape for LdRoute {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="19" r="3" />
  <path d="M9 19h8.5a3.5 3.5 0 0 0 0-7h-11a3.5 3.5 0 0 1 0-7H15" />
  <circle cx="18" cy="5" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRouter;
impl IconShape for LdRouter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="8" x="2" y="14" rx="2" />
  <path d="M6.01 18H6" />
  <path d="M10.01 18H10" />
  <path d="M15 10v4" />
  <path d="M17.84 7.17a4 4 0 0 0-5.66 0" />
  <path d="M20.66 4.34a8 8 0 0 0-11.31 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRows2;
impl IconShape for LdRows2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 12h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRows3;
impl IconShape for LdRows3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M21 9H3" />
  <path d="M21 15H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRows4;
impl IconShape for LdRows4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M21 7.5H3" />
  <path d="M21 12H3" />
  <path d="M21 16.5H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRss;
impl IconShape for LdRss {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 11a9 9 0 0 1 9 9" />
  <path d="M4 4a16 16 0 0 1 16 16" />
  <circle cx="5" cy="19" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRuler;
impl IconShape for LdRuler {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21.3 15.3a2.4 2.4 0 0 1 0 3.4l-2.6 2.6a2.4 2.4 0 0 1-3.4 0L2.7 8.7a2.41 2.41 0 0 1 0-3.4l2.6-2.6a2.41 2.41 0 0 1 3.4 0Z" />
  <path d="m14.5 12.5 2-2" />
  <path d="m11.5 9.5 2-2" />
  <path d="m8.5 6.5 2-2" />
  <path d="m17.5 15.5 2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdRussianRuble;
impl IconShape for LdRussianRuble {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 11h8a4 4 0 0 0 0-8H9v18" />
  <path d="M6 15h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSailboat;
impl IconShape for LdSailboat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 18H2a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4Z" />
  <path d="M21 14 10 2 3 14h18Z" />
  <path d="M10 2v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSalad;
impl IconShape for LdSalad {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 21h10" />
  <path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z" />
  <path d="M11.38 12a2.4 2.4 0 0 1-.4-4.77 2.4 2.4 0 0 1 3.2-2.77 2.4 2.4 0 0 1 3.47-.63 2.4 2.4 0 0 1 3.37 3.37 2.4 2.4 0 0 1-1.1 3.7 2.51 2.51 0 0 1 .03 1.1" />
  <path d="m13 12 4-4" />
  <path d="M10.9 7.25A3.99 3.99 0 0 0 4 10c0 .73.2 1.41.54 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSandwich;
impl IconShape for LdSandwich {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 11v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-3" />
  <path d="M12 19H4a1 1 0 0 1-1-1v-2a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3.83" />
  <path d="m3 11 7.77-6.04a2 2 0 0 1 2.46 0L21 11H3Z" />
  <path d="M12.97 19.77 7 15h12.5l-3.75 4.5a2 2 0 0 1-2.78.27Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSatelliteDish;
impl IconShape for LdSatelliteDish {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 10a7.31 7.31 0 0 0 10 10Z" />
  <path d="m9 15 3-3" />
  <path d="M17 13a6 6 0 0 0-6-6" />
  <path d="M21 13A10 10 0 0 0 11 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSatellite;
impl IconShape for LdSatellite {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 7 9 3 5 7l4 4" />
  <path d="m17 11 4 4-4 4-4-4" />
  <path d="m8 12 4 4 6-6-4-4Z" />
  <path d="m16 8 3-3" />
  <path d="M9 21a6 6 0 0 0-6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSaveAll;
impl IconShape for LdSaveAll {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 4a2 2 0 0 1 2-2h10l4 4v10.2a2 2 0 0 1-2 1.8H8a2 2 0 0 1-2-2Z" />
  <path d="M10 2v4h6" />
  <path d="M18 18v-7h-8v7" />
  <path d="M18 22H4a2 2 0 0 1-2-2V6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSave;
impl IconShape for LdSave {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z" />
  <path d="M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7" />
  <path d="M7 3v4a1 1 0 0 0 1 1h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScale3d;
impl IconShape for LdScale3d {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="19" cy="19" r="2" />
  <circle cx="5" cy="5" r="2" />
  <path d="M5 7v12h12" />
  <path d="m5 19 6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScale;
impl IconShape for LdScale {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z" />
  <path d="m2 16 3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1Z" />
  <path d="M7 21h10" />
  <path d="M12 3v18" />
  <path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScaling;
impl IconShape for LdScaling {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
  <path d="M14 15H9v-5" />
  <path d="M16 3h5v5" />
  <path d="M21 3 9 15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScanBarcode;
impl IconShape for LdScanBarcode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <path d="M8 7v10" />
  <path d="M12 7v10" />
  <path d="M17 7v10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScanEye;
impl IconShape for LdScanEye {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <circle cx="12" cy="12" r="1" />
  <path d="M5 12s2.5-5 7-5 7 5 7 5-2.5 5-7 5-7-5-7-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScanFace;
impl IconShape for LdScanFace {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <path d="M8 14s1.5 2 4 2 4-2 4-2" />
  <path d="M9 9h.01" />
  <path d="M15 9h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScanLine;
impl IconShape for LdScanLine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <path d="M7 12h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScanSearch;
impl IconShape for LdScanSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <circle cx="12" cy="12" r="3" />
  <path d="m16 16-1.9-1.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScanText;
impl IconShape for LdScanText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
  <path d="M7 8h8" />
  <path d="M7 12h10" />
  <path d="M7 16h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScan;
impl IconShape for LdScan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7V5a2 2 0 0 1 2-2h2" />
  <path d="M17 3h2a2 2 0 0 1 2 2v2" />
  <path d="M21 17v2a2 2 0 0 1-2 2h-2" />
  <path d="M7 21H5a2 2 0 0 1-2-2v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScatterChart;
impl IconShape for LdScatterChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
  <circle cx="18.5" cy="5.5" r=".5" fill="currentColor" />
  <circle cx="11.5" cy="11.5" r=".5" fill="currentColor" />
  <circle cx="7.5" cy="16.5" r=".5" fill="currentColor" />
  <circle cx="17.5" cy="14.5" r=".5" fill="currentColor" />
  <path d="M3 3v18h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSchool;
impl IconShape for LdSchool {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 22v-4a2 2 0 1 0-4 0v4" />
  <path d="m18 10 4 2v8a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8l4-2" />
  <path d="M18 5v17" />
  <path d="m4 6 8-4 8 4" />
  <path d="M6 5v17" />
  <circle cx="12" cy="9" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScissorsLineDashed;
impl IconShape for LdScissorsLineDashed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5.42 9.42 8 12" />
  <circle cx="4" cy="8" r="2" />
  <path d="m14 6-8.58 8.58" />
  <circle cx="4" cy="16" r="2" />
  <path d="M10.8 14.8 14 18" />
  <path d="M16 12h-2" />
  <path d="M22 12h-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScissors;
impl IconShape for LdScissors {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="6" r="3" />
  <path d="M8.12 8.12 12 12" />
  <path d="M20 4 8.12 15.88" />
  <circle cx="6" cy="18" r="3" />
  <path d="M14.8 14.8 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScreenShareOff;
impl IconShape for LdScreenShareOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3" />
  <path d="M8 21h8" />
  <path d="M12 17v4" />
  <path d="m22 3-5 5" />
  <path d="m17 3 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScreenShare;
impl IconShape for LdScreenShare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3" />
  <path d="M8 21h8" />
  <path d="M12 17v4" />
  <path d="m17 8 5-5" />
  <path d="M17 3h5v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScrollText;
impl IconShape for LdScrollText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 12h-5" />
  <path d="M15 8h-5" />
  <path d="M19 17V5a2 2 0 0 0-2-2H4" />
  <path d="M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdScroll;
impl IconShape for LdScroll {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 17V5a2 2 0 0 0-2-2H4" />
  <path d="M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSearchCheck;
impl IconShape for LdSearchCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8 11 2 2 4-4" />
  <circle cx="11" cy="11" r="8" />
  <path d="m21 21-4.3-4.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSearchCode;
impl IconShape for LdSearchCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 9-2 2 2 2" />
  <path d="m13 13 2-2-2-2" />
  <circle cx="11" cy="11" r="8" />
  <path d="m21 21-4.3-4.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSearchSlash;
impl IconShape for LdSearchSlash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m13.5 8.5-5 5" />
  <circle cx="11" cy="11" r="8" />
  <path d="m21 21-4.3-4.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSearchX;
impl IconShape for LdSearchX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m13.5 8.5-5 5" />
  <path d="m8.5 8.5 5 5" />
  <circle cx="11" cy="11" r="8" />
  <path d="m21 21-4.3-4.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSearch;
impl IconShape for LdSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11" cy="11" r="8" />
  <path d="m21 21-4.3-4.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSendHorizontal;
impl IconShape for LdSendHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m3 3 3 9-3 9 19-9Z" />
  <path d="M6 12h16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSendToBack;
impl IconShape for LdSendToBack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect x="14" y="14" width="8" height="8" rx="2" />
  <rect x="2" y="2" width="8" height="8" rx="2" />
  <path d="M7 14v1a2 2 0 0 0 2 2h1" />
  <path d="M14 7h1a2 2 0 0 1 2 2v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSend;
impl IconShape for LdSend {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m22 2-7 20-4-9-9-4Z" />
  <path d="M22 2 11 13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSeparatorHorizontal;
impl IconShape for LdSeparatorHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="3" x2="21" y1="12" y2="12" />
  <polyline points="8 8 12 4 16 8" />
  <polyline points="16 16 12 20 8 16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSeparatorVertical;
impl IconShape for LdSeparatorVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="12" x2="12" y1="3" y2="21" />
  <polyline points="8 8 4 12 8 16" />
  <polyline points="16 16 20 12 16 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdServerCog;
impl IconShape for LdServerCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="3" />
  <path d="M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5" />
  <path d="M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5" />
  <path d="M6 6h.01" />
  <path d="M6 18h.01" />
  <path d="m15.7 13.4-.9-.3" />
  <path d="m9.2 10.9-.9-.3" />
  <path d="m10.6 15.7.3-.9" />
  <path d="m13.6 15.7-.4-1" />
  <path d="m10.8 9.3-.4-1" />
  <path d="m8.3 13.6 1-.4" />
  <path d="m14.7 10.8 1-.4" />
  <path d="m13.4 8.3-.3.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdServerCrash;
impl IconShape for LdServerCrash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-2" />
  <path d="M6 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2" />
  <path d="M6 6h.01" />
  <path d="M6 18h.01" />
  <path d="m13 6-4 6h6l-4 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdServerOff;
impl IconShape for LdServerOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5" />
  <path d="M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z" />
  <path d="M22 17v-1a2 2 0 0 0-2-2h-1" />
  <path d="M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z" />
  <path d="M6 18h.01" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdServer;
impl IconShape for LdServer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="8" x="2" y="2" rx="2" ry="2" />
  <rect width="20" height="8" x="2" y="14" rx="2" ry="2" />
  <line x1="6" x2="6.01" y1="6" y2="6" />
  <line x1="6" x2="6.01" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSettings2;
impl IconShape for LdSettings2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 7h-9" />
  <path d="M14 17H5" />
  <circle cx="17" cy="17" r="3" />
  <circle cx="7" cy="7" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSettings;
impl IconShape for LdSettings {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
  <circle cx="12" cy="12" r="3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShapes;
impl IconShape for LdShapes {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.3 10a.7.7 0 0 1-.626-1.079L11.4 3a.7.7 0 0 1 1.198-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z" />
  <rect x="3" y="14" width="7" height="7" rx="1" />
  <circle cx="17.5" cy="17.5" r="3.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShare2;
impl IconShape for LdShare2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="5" r="3" />
  <circle cx="6" cy="12" r="3" />
  <circle cx="18" cy="19" r="3" />
  <line x1="8.59" x2="15.42" y1="13.51" y2="17.49" />
  <line x1="15.41" x2="8.59" y1="6.51" y2="10.49" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShare;
impl IconShape for LdShare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
  <polyline points="16 6 12 2 8 6" />
  <line x1="12" x2="12" y1="2" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSheet;
impl IconShape for LdSheet {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <line x1="3" x2="21" y1="9" y2="9" />
  <line x1="3" x2="21" y1="15" y2="15" />
  <line x1="9" x2="9" y1="9" y2="21" />
  <line x1="15" x2="15" y1="9" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShell;
impl IconShape for LdShell {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 11a2 2 0 1 1-4 0 4 4 0 0 1 8 0 6 6 0 0 1-12 0 8 8 0 0 1 16 0 10 10 0 1 1-20 0 11.93 11.93 0 0 1 2.42-7.22 2 2 0 1 1 3.16 2.44" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldAlert;
impl IconShape for LdShieldAlert {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="M12 8v4" />
  <path d="M12 16h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldBan;
impl IconShape for LdShieldBan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="m4.243 5.21 14.39 12.472" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldCheck;
impl IconShape for LdShieldCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="m9 12 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldEllipsis;
impl IconShape for LdShieldEllipsis {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="M8 12h.01" />
  <path d="M12 12h.01" />
  <path d="M16 12h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldHalf;
impl IconShape for LdShieldHalf {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="M12 22V2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldMinus;
impl IconShape for LdShieldMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="M9 12h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldOff;
impl IconShape for LdShieldOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 2 20 20" />
  <path d="M5 5a1 1 0 0 0-1 1v7c0 5 3.5 7.5 7.67 8.94a1 1 0 0 0 .67.01c2.35-.82 4.48-1.97 5.9-3.71" />
  <path d="M9.309 3.652A12.252 12.252 0 0 0 11.24 2.28a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v7a9.784 9.784 0 0 1-.08 1.264" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldPlus;
impl IconShape for LdShieldPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="M9 12h6" />
  <path d="M12 9v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldQuestion;
impl IconShape for LdShieldQuestion {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3" />
  <path d="M12 17h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShieldX;
impl IconShape for LdShieldX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
  <path d="m14.5 9.5-5 5" />
  <path d="m9.5 9.5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShield;
impl IconShape for LdShield {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShipWheel;
impl IconShape for LdShipWheel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="8" />
  <path d="M12 2v7.5" />
  <path d="m19 5-5.23 5.23" />
  <path d="M22 12h-7.5" />
  <path d="m19 19-5.23-5.23" />
  <path d="M12 14.5V22" />
  <path d="M10.23 13.77 5 19" />
  <path d="M9.5 12H2" />
  <path d="M10.23 10.23 5 5" />
  <circle cx="12" cy="12" r="2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShip;
impl IconShape for LdShip {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1 .6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
  <path d="M19.38 20A11.6 11.6 0 0 0 21 14l-9-4-9 4c0 2.9.94 5.34 2.81 7.76" />
  <path d="M19 13V7a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v6" />
  <path d="M12 10v4" />
  <path d="M12 2v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShirt;
impl IconShape for LdShirt {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20.38 3.46 16 2a4 4 0 0 1-8 0L3.62 3.46a2 2 0 0 0-1.34 2.23l.58 3.47a1 1 0 0 0 .99.84H6v10c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2V10h2.15a1 1 0 0 0 .99-.84l.58-3.47a2 2 0 0 0-1.34-2.23z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShoppingBag;
impl IconShape for LdShoppingBag {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 2 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4Z" />
  <path d="M3 6h18" />
  <path d="M16 10a4 4 0 0 1-8 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShoppingBasket;
impl IconShape for LdShoppingBasket {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 11-1 9" />
  <path d="m19 11-4-7" />
  <path d="M2 11h20" />
  <path d="m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8a2 2 0 0 0 2-1.6l1.7-7.4" />
  <path d="M4.5 15.5h15" />
  <path d="m5 11 4-7" />
  <path d="m9 11 1 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShoppingCart;
impl IconShape for LdShoppingCart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="8" cy="21" r="1" />
  <circle cx="19" cy="21" r="1" />
  <path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShovel;
impl IconShape for LdShovel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 22v-5l5-5 5 5-5 5z" />
  <path d="M9.5 14.5 16 8" />
  <path d="m17 2 5 5-.5.5a3.53 3.53 0 0 1-5 0s0 0 0 0a3.53 3.53 0 0 1 0-5L17 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShowerHead;
impl IconShape for LdShowerHead {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m4 4 2.5 2.5" />
  <path d="M13.5 6.5a4.95 4.95 0 0 0-7 7" />
  <path d="M15 5 5 15" />
  <path d="M14 17v.01" />
  <path d="M10 16v.01" />
  <path d="M13 13v.01" />
  <path d="M16 10v.01" />
  <path d="M11 20v.01" />
  <path d="M17 14v.01" />
  <path d="M20 11v.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShrink;
impl IconShape for LdShrink {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 15 6 6m-6-6v4.8m0-4.8h4.8" />
  <path d="M9 19.8V15m0 0H4.2M9 15l-6 6" />
  <path d="M15 4.2V9m0 0h4.8M15 9l6-6" />
  <path d="M9 4.2V9m0 0H4.2M9 9 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShrub;
impl IconShape for LdShrub {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22v-7l-2-2" />
  <path d="M17 8v.8A6 6 0 0 1 13.8 20v0H10v0A6.5 6.5 0 0 1 7 8h0a5 5 0 0 1 10 0Z" />
  <path d="m14 14-2 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdShuffle;
impl IconShape for LdShuffle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22" />
  <path d="m18 2 4 4-4 4" />
  <path d="M2 6h1.9c1.5 0 2.9.9 3.6 2.2" />
  <path d="M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8" />
  <path d="m18 14 4 4-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSigma;
impl IconShape for LdSigma {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 7V4H6l6 8-6 8h12v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignalHigh;
impl IconShape for LdSignalHigh {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20h.01" />
  <path d="M7 20v-4" />
  <path d="M12 20v-8" />
  <path d="M17 20V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignalLow;
impl IconShape for LdSignalLow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20h.01" />
  <path d="M7 20v-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignalMedium;
impl IconShape for LdSignalMedium {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20h.01" />
  <path d="M7 20v-4" />
  <path d="M12 20v-8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignalZero;
impl IconShape for LdSignalZero {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignal;
impl IconShape for LdSignal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 20h.01" />
  <path d="M7 20v-4" />
  <path d="M12 20v-8" />
  <path d="M17 20V8" />
  <path d="M22 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignpostBig;
impl IconShape for LdSignpostBig {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 9H4L2 7l2-2h6" />
  <path d="M14 5h6l2 2-2 2h-6" />
  <path d="M10 22V4a2 2 0 1 1 4 0v18" />
  <path d="M8 22h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSignpost;
impl IconShape for LdSignpost {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3v3" />
  <path d="M18.5 13h-13L2 9.5 5.5 6h13L22 9.5Z" />
  <path d="M12 13v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSiren;
impl IconShape for LdSiren {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 18v-6a5 5 0 1 1 10 0v6" />
  <path d="M5 21a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2z" />
  <path d="M21 12h1" />
  <path d="M18.5 4.5 18 5" />
  <path d="M2 12h1" />
  <path d="M12 2v1" />
  <path d="m4.929 4.929.707.707" />
  <path d="M12 12v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSkipBack;
impl IconShape for LdSkipBack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="19 20 9 12 19 4 19 20" />
  <line x1="5" x2="5" y1="19" y2="5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSkipForward;
impl IconShape for LdSkipForward {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="5 4 15 12 5 20 5 4" />
  <line x1="19" x2="19" y1="5" y2="19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSkull;
impl IconShape for LdSkull {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="9" cy="12" r="1" />
  <circle cx="15" cy="12" r="1" />
  <path d="M8 20v2h8v-2" />
  <path d="m12.5 17-.5-1-.5 1h1z" />
  <path d="M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSlack;
impl IconShape for LdSlack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="3" height="8" x="13" y="2" rx="1.5" />
  <path d="M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" />
  <rect width="3" height="8" x="8" y="14" rx="1.5" />
  <path d="M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" />
  <rect width="8" height="3" x="14" y="13" rx="1.5" />
  <path d="M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" />
  <rect width="8" height="3" x="2" y="8" rx="1.5" />
  <path d="M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSlash;
impl IconShape for LdSlash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 2 2 22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSlice;
impl IconShape for LdSlice {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m8 14-6 6h9v-3" />
  <path d="M18.37 3.63 8 14l3 3L21.37 6.63a2.12 2.12 0 1 0-3-3Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSlidersHorizontal;
impl IconShape for LdSlidersHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="21" x2="14" y1="4" y2="4" />
  <line x1="10" x2="3" y1="4" y2="4" />
  <line x1="21" x2="12" y1="12" y2="12" />
  <line x1="8" x2="3" y1="12" y2="12" />
  <line x1="21" x2="16" y1="20" y2="20" />
  <line x1="12" x2="3" y1="20" y2="20" />
  <line x1="14" x2="14" y1="2" y2="6" />
  <line x1="8" x2="8" y1="10" y2="14" />
  <line x1="16" x2="16" y1="18" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSlidersVertical;
impl IconShape for LdSlidersVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="4" x2="4" y1="21" y2="14" />
  <line x1="4" x2="4" y1="10" y2="3" />
  <line x1="12" x2="12" y1="21" y2="12" />
  <line x1="12" x2="12" y1="8" y2="3" />
  <line x1="20" x2="20" y1="21" y2="16" />
  <line x1="20" x2="20" y1="12" y2="3" />
  <line x1="2" x2="6" y1="14" y2="14" />
  <line x1="10" x2="14" y1="8" y2="8" />
  <line x1="18" x2="22" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSmartphoneCharging;
impl IconShape for LdSmartphoneCharging {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
  <path d="M12.667 8 10 12h4l-2.667 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSmartphoneNfc;
impl IconShape for LdSmartphoneNfc {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="7" height="12" x="2" y="6" rx="1" />
  <path d="M13 8.32a7.43 7.43 0 0 1 0 7.36" />
  <path d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58" />
  <path d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSmartphone;
impl IconShape for LdSmartphone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="14" height="20" x="5" y="2" rx="2" ry="2" />
  <path d="M12 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSmilePlus;
impl IconShape for LdSmilePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 11v1a10 10 0 1 1-9-10" />
  <path d="M8 14s1.5 2 4 2 4-2 4-2" />
  <line x1="9" x2="9.01" y1="9" y2="9" />
  <line x1="15" x2="15.01" y1="9" y2="9" />
  <path d="M16 5h6" />
  <path d="M19 2v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSmile;
impl IconShape for LdSmile {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <path d="M8 14s1.5 2 4 2 4-2 4-2" />
  <line x1="9" x2="9.01" y1="9" y2="9" />
  <line x1="15" x2="15.01" y1="9" y2="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSnail;
impl IconShape for LdSnail {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0" />
  <circle cx="10" cy="13" r="8" />
  <path d="M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6" />
  <path d="M18 3 19.1 5.2" />
  <path d="M22 3 20.9 5.2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSnowflake;
impl IconShape for LdSnowflake {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="2" x2="22" y1="12" y2="12" />
  <line x1="12" x2="12" y1="2" y2="22" />
  <path d="m20 16-4-4 4-4" />
  <path d="m4 8 4 4-4 4" />
  <path d="m16 4-4 4-4-4" />
  <path d="m8 20 4-4 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSofa;
impl IconShape for LdSofa {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3" />
  <path d="M2 11v5a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H6v-2a2 2 0 0 0-4 0Z" />
  <path d="M4 18v2" />
  <path d="M20 18v2" />
  <path d="M12 4v9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSoup;
impl IconShape for LdSoup {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 21a9 9 0 0 0 9-9H3a9 9 0 0 0 9 9Z" />
  <path d="M7 21h10" />
  <path d="M19.5 12 22 6" />
  <path d="M16.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.73 1.62" />
  <path d="M11.25 3c.27.1.8.53.74 1.36-.05.83-.93 1.2-.98 2.02-.06.78.33 1.24.72 1.62" />
  <path d="M6.25 3c.27.1.8.53.75 1.36-.06.83-.93 1.2-1 2.02-.05.78.34 1.24.74 1.62" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpace;
impl IconShape for LdSpace {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpade;
impl IconShape for LdSpade {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 9c-1.5 1.5-3 3.2-3 5.5A5.5 5.5 0 0 0 7.5 20c1.8 0 3-.5 4.5-2 1.5 1.5 2.7 2 4.5 2a5.5 5.5 0 0 0 5.5-5.5c0-2.3-1.5-4-3-5.5l-7-7-7 7Z" />
  <path d="M12 18v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSparkle;
impl IconShape for LdSparkle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 3-1.9 5.8a2 2 0 0 1-1.287 1.288L3 12l5.8 1.9a2 2 0 0 1 1.288 1.287L12 21l1.9-5.8a2 2 0 0 1 1.287-1.288L21 12l-5.8-1.9a2 2 0 0 1-1.288-1.287Z" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSparkles;
impl IconShape for LdSparkles {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z" />
  <path d="M5 3v4" />
  <path d="M19 17v4" />
  <path d="M3 5h4" />
  <path d="M17 19h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpeaker;
impl IconShape for LdSpeaker {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="20" x="4" y="2" rx="2" />
  <path d="M12 6h.01" />
  <circle cx="12" cy="14" r="4" />
  <path d="M12 14h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpeech;
impl IconShape for LdSpeech {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.8 20v-4.1l1.9.2a2.3 2.3 0 0 0 2.164-2.1V8.3A5.37 5.37 0 0 0 2 8.25c0 2.8.656 3.054 1 4.55a5.77 5.77 0 0 1 .029 2.758L2 20" />
  <path d="M19.8 17.8a7.5 7.5 0 0 0 .003-10.603" />
  <path d="M17 15a3.5 3.5 0 0 0-.025-4.975" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpellCheck2;
impl IconShape for LdSpellCheck2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 16 6-12 6 12" />
  <path d="M8 12h8" />
  <path d="M4 21c1.1 0 1.1-1 2.3-1s1.1 1 2.3 1c1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpellCheck;
impl IconShape for LdSpellCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m6 16 6-12 6 12" />
  <path d="M8 12h8" />
  <path d="m16 20 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSpline;
impl IconShape for LdSpline {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="19" cy="5" r="2" />
  <circle cx="5" cy="19" r="2" />
  <path d="M5 17A12 12 0 0 1 17 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSplit;
impl IconShape for LdSplit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 3h5v5" />
  <path d="M8 3H3v5" />
  <path d="M12 22v-8.3a4 4 0 0 0-1.172-2.872L3 3" />
  <path d="m15 9 6-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSprayCan;
impl IconShape for LdSprayCan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 3h.01" />
  <path d="M7 5h.01" />
  <path d="M11 7h.01" />
  <path d="M3 7h.01" />
  <path d="M7 9h.01" />
  <path d="M3 11h.01" />
  <rect width="4" height="4" x="15" y="5" />
  <path d="m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2" />
  <path d="m13 14 8-2" />
  <path d="m13 19 8-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSprout;
impl IconShape for LdSprout {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 20h10" />
  <path d="M10 20c5.5-2.5.8-6.4 3-10" />
  <path d="M9.5 9.4c1.1.8 1.8 2.2 2.3 3.7-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2 2.8-.5 4.4 0 5.5.8z" />
  <path d="M14.1 6a7 7 0 0 0-1.1 4c1.9-.1 3.3-.6 4.3-1.4 1-1 1.6-2.3 1.7-4.6-2.7.1-4 1-4.9 2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareActivity;
impl IconShape for LdSquareActivity {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M17 12h-2l-2 5-2-10-2 5H7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowDownLeft;
impl IconShape for LdSquareArrowDownLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m16 8-8 8" />
  <path d="M16 16H8V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowDownRight;
impl IconShape for LdSquareArrowDownRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m8 8 8 8" />
  <path d="M16 8v8H8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowDown;
impl IconShape for LdSquareArrowDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M12 8v8" />
  <path d="m8 12 4 4 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowLeft;
impl IconShape for LdSquareArrowLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m12 8-4 4 4 4" />
  <path d="M16 12H8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowOutDownLeft;
impl IconShape for LdSquareArrowOutDownLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 21h6a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6" />
  <path d="m3 21 9-9" />
  <path d="M9 21H3v-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowOutDownRight;
impl IconShape for LdSquareArrowOutDownRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" />
  <path d="m21 21-9-9" />
  <path d="M21 15v6h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowOutUpLeft;
impl IconShape for LdSquareArrowOutUpLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-6" />
  <path d="m3 3 9 9" />
  <path d="M3 9V3h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowOutUpRight;
impl IconShape for LdSquareArrowOutUpRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6" />
  <path d="m21 3-9 9" />
  <path d="M15 3h6v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowRight;
impl IconShape for LdSquareArrowRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 12h8" />
  <path d="m12 16 4-4-4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowUpLeft;
impl IconShape for LdSquareArrowUpLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 16V8h8" />
  <path d="M16 16 8 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowUpRight;
impl IconShape for LdSquareArrowUpRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 8h8v8" />
  <path d="m8 16 8-8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareArrowUp;
impl IconShape for LdSquareArrowUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m16 12-4-4-4 4" />
  <path d="M12 16V8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareAsterisk;
impl IconShape for LdSquareAsterisk {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M12 8v8" />
  <path d="m8.5 14 7-4" />
  <path d="m8.5 10 7 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareBottomDashedScissors;
impl IconShape for LdSquareBottomDashedScissors {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2" />
  <path d="M10 22H8" />
  <path d="M16 22h-2" />
  <circle cx="8" cy="8" r="2" />
  <path d="M9.414 9.414 12 12" />
  <path d="M14.8 14.8 18 18" />
  <circle cx="8" cy="16" r="2" />
  <path d="m18 6-8.586 8.586" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareCheckBig;
impl IconShape for LdSquareCheckBig {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 11 3 3L22 4" />
  <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareCheck;
impl IconShape for LdSquareCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m9 12 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareChevronDown;
impl IconShape for LdSquareChevronDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m16 10-4 4-4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareChevronLeft;
impl IconShape for LdSquareChevronLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m14 16-4-4 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareChevronRight;
impl IconShape for LdSquareChevronRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m10 8 4 4-4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareChevronUp;
impl IconShape for LdSquareChevronUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m8 14 4-4 4 4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareCode;
impl IconShape for LdSquareCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m10 10-2 2 2 2" />
  <path d="m14 14 2-2-2-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareDashedBottomCode;
impl IconShape for LdSquareDashedBottomCode {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m10 10-2 2 2 2" />
  <path d="m14 14 2-2-2-2" />
  <path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2" />
  <path d="M9 21h1" />
  <path d="M14 21h1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareDashedBottom;
impl IconShape for LdSquareDashedBottom {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2" />
  <path d="M9 21h1" />
  <path d="M14 21h1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareDashedKanban;
impl IconShape for LdSquareDashedKanban {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 7v7" />
  <path d="M12 7v4" />
  <path d="M16 7v9" />
  <path d="M5 3a2 2 0 0 0-2 2" />
  <path d="M9 3h1" />
  <path d="M14 3h1" />
  <path d="M19 3a2 2 0 0 1 2 2" />
  <path d="M21 9v1" />
  <path d="M21 14v1" />
  <path d="M21 19a2 2 0 0 1-2 2" />
  <path d="M14 21h1" />
  <path d="M9 21h1" />
  <path d="M5 21a2 2 0 0 1-2-2" />
  <path d="M3 14v1" />
  <path d="M3 9v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareDashedMousePointer;
impl IconShape for LdSquareDashedMousePointer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 3a2 2 0 0 0-2 2" />
  <path d="M19 3a2 2 0 0 1 2 2" />
  <path d="m12 12 4 10 1.7-4.3L22 16Z" />
  <path d="M5 21a2 2 0 0 1-2-2" />
  <path d="M9 3h1" />
  <path d="M9 21h2" />
  <path d="M14 3h1" />
  <path d="M3 9v1" />
  <path d="M21 9v2" />
  <path d="M3 14v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareDivide;
impl IconShape for LdSquareDivide {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <line x1="8" x2="16" y1="12" y2="12" />
  <line x1="12" x2="12" y1="16" y2="16" />
  <line x1="12" x2="12" y1="8" y2="8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareDot;
impl IconShape for LdSquareDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <circle cx="12" cy="12" r="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareEqual;
impl IconShape for LdSquareEqual {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7 10h10" />
  <path d="M7 14h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareFunction;
impl IconShape for LdSquareFunction {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3" />
  <path d="M9 11.2h5.7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareGanttChart;
impl IconShape for LdSquareGanttChart {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 8h7" />
  <path d="M8 12h6" />
  <path d="M11 16h5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareKanban;
impl IconShape for LdSquareKanban {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 7v7" />
  <path d="M12 7v4" />
  <path d="M16 7v9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareLibrary;
impl IconShape for LdSquareLibrary {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7 7v10" />
  <path d="M11 7v10" />
  <path d="m15 7 2 10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareM;
impl IconShape for LdSquareM {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 16V8l4 4 4-4v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareMenu;
impl IconShape for LdSquareMenu {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7 8h10" />
  <path d="M7 12h10" />
  <path d="M7 16h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareMinus;
impl IconShape for LdSquareMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 12h8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareMousePointer;
impl IconShape for LdSquareMousePointer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6" />
  <path d="m12 12 4 10 1.7-4.3L22 16Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareParkingOff;
impl IconShape for LdSquareParkingOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.6 3.6A2 2 0 0 1 5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-.59 1.41" />
  <path d="M3 8.7V19a2 2 0 0 0 2 2h10.3" />
  <path d="m2 2 20 20" />
  <path d="M13 13a3 3 0 1 0 0-6H9v2" />
  <path d="M9 17v-2.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareParking;
impl IconShape for LdSquareParking {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M9 17V7h4a3 3 0 0 1 0 6H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePen;
impl IconShape for LdSquarePen {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
  <path d="M18.375 2.625a2.121 2.121 0 1 1 3 3L12 15l-4 1 1-4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePercent;
impl IconShape for LdSquarePercent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m15 9-6 6" />
  <path d="M9 9h.01" />
  <path d="M15 15h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePi;
impl IconShape for LdSquarePi {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M7 7h10" />
  <path d="M10 7v10" />
  <path d="M16 17a2 2 0 0 1-2-2V7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePilcrow;
impl IconShape for LdSquarePilcrow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M12 12H9.5a2.5 2.5 0 0 1 0-5H17" />
  <path d="M12 7v10" />
  <path d="M16 7v10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePlay;
impl IconShape for LdSquarePlay {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="m9 8 6 4-6 4Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePlus;
impl IconShape for LdSquarePlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M8 12h8" />
  <path d="M12 8v8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquarePower;
impl IconShape for LdSquarePower {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M12 7v5" />
  <path d="M8 9a5.14 5.14 0 0 0 4 8 4.95 4.95 0 0 0 4-8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareRadical;
impl IconShape for LdSquareRadical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 12h2l2 5 2-10h4" />
  <rect x="3" y="3" width="18" height="18" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareScissors;
impl IconShape for LdSquareScissors {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="20" x="2" y="2" rx="2" />
  <circle cx="8" cy="8" r="2" />
  <path d="M9.414 9.414 12 12" />
  <path d="M14.8 14.8 18 18" />
  <circle cx="8" cy="16" r="2" />
  <path d="m18 6-8.586 8.586" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareSigma;
impl IconShape for LdSquareSigma {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M16 8.9V7H8l4 5-4 5h8v-1.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareSlash;
impl IconShape for LdSquareSlash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <line x1="9" x2="15" y1="15" y2="9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareSplitHorizontal;
impl IconShape for LdSquareSplitHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 19H5c-1 0-2-1-2-2V7c0-1 1-2 2-2h3" />
  <path d="M16 5h3c1 0 2 1 2 2v10c0 1-1 2-2 2h-3" />
  <line x1="12" x2="12" y1="4" y2="20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareSplitVertical;
impl IconShape for LdSquareSplitVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 8V5c0-1 1-2 2-2h10c1 0 2 1 2 2v3" />
  <path d="M19 16v3c0 1-1 2-2 2H7c-1 0-2-1-2-2v-3" />
  <line x1="4" x2="20" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareStack;
impl IconShape for LdSquareStack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 10c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2" />
  <path d="M10 16c-1.1 0-2-.9-2-2v-4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2" />
  <rect width="8" height="8" x="14" y="14" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareTerminal;
impl IconShape for LdSquareTerminal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m7 11 2-2-2-2" />
  <path d="M11 13h4" />
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareUserRound;
impl IconShape for LdSquareUserRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 21a6 6 0 0 0-12 0" />
  <circle cx="12" cy="11" r="4" />
  <rect width="18" height="18" x="3" y="3" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareUser;
impl IconShape for LdSquareUser {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <circle cx="12" cy="10" r="3" />
  <path d="M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquareX;
impl IconShape for LdSquareX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <path d="m15 9-6 6" />
  <path d="m9 9 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquare;
impl IconShape for LdSquare {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquircle;
impl IconShape for LdSquircle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3c7.2 0 9 1.8 9 9s-1.8 9-9 9-9-1.8-9-9 1.8-9 9-9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSquirrel;
impl IconShape for LdSquirrel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.236 22a3 3 0 0 0-2.2-5" />
  <path d="M16 20a3 3 0 0 1 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4" />
  <path d="M18 13h.01" />
  <path d="M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStamp;
impl IconShape for LdStamp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 22h14" />
  <path d="M19.27 13.73A2.5 2.5 0 0 0 17.5 13h-11A2.5 2.5 0 0 0 4 15.5V17a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1v-1.5c0-.66-.26-1.3-.73-1.77Z" />
  <path d="M14 13V8.5C14 7 15 7 15 5a3 3 0 0 0-3-3c-1.66 0-3 1-3 3s1 2 1 3.5V13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStarHalf;
impl IconShape for LdStarHalf {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 17.8 5.8 21 7 14.1 2 9.3l7-1L12 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStarOff;
impl IconShape for LdStarOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8.34 8.34 2 9.27l5 4.87L5.82 21 12 17.77 18.18 21l-.59-3.43" />
  <path d="M18.42 12.76 22 9.27l-6.91-1L12 2l-1.44 2.91" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStar;
impl IconShape for LdStar {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStepBack;
impl IconShape for LdStepBack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="18" x2="18" y1="20" y2="4" />
  <polygon points="14,20 4,12 14,4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStepForward;
impl IconShape for LdStepForward {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="6" x2="6" y1="4" y2="20" />
  <polygon points="10,4 20,12 10,20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStethoscope;
impl IconShape for LdStethoscope {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4.8 2.3A.3.3 0 1 0 5 2H4a2 2 0 0 0-2 2v5a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6V4a2 2 0 0 0-2-2h-1a.2.2 0 1 0 .3.3" />
  <path d="M8 15v1a6 6 0 0 0 6 6v0a6 6 0 0 0 6-6v-4" />
  <circle cx="20" cy="10" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSticker;
impl IconShape for LdSticker {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15.5 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3Z" />
  <path d="M14 3v4a2 2 0 0 0 2 2h4" />
  <path d="M8 13h0" />
  <path d="M16 13h0" />
  <path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStickyNote;
impl IconShape for LdStickyNote {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8Z" />
  <path d="M15 3v4a2 2 0 0 0 2 2h4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStore;
impl IconShape for LdStore {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7" />
  <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
  <path d="M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4" />
  <path d="M2 7h20" />
  <path d="M22 7v3a2 2 0 0 1-2 2v0a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 16 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 12 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 8 12a2.7 2.7 0 0 1-1.59-.63.7.7 0 0 0-.82 0A2.7 2.7 0 0 1 4 12v0a2 2 0 0 1-2-2V7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStretchHorizontal;
impl IconShape for LdStretchHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="6" x="2" y="4" rx="2" />
  <rect width="20" height="6" x="2" y="14" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStretchVertical;
impl IconShape for LdStretchVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="6" height="20" x="4" y="2" rx="2" />
  <rect width="6" height="20" x="14" y="2" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdStrikethrough;
impl IconShape for LdStrikethrough {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 4H9a3 3 0 0 0-2.83 4" />
  <path d="M14 12a4 4 0 0 1 0 8H6" />
  <line x1="4" x2="20" y1="12" y2="12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSubscript;
impl IconShape for LdSubscript {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m4 5 8 8" />
  <path d="m12 5-8 8" />
  <path d="M20 19h-4c0-1.5.44-2 1.5-2.5S20 15.33 20 14c0-.47-.17-.93-.48-1.29a2.11 2.11 0 0 0-2.62-.44c-.42.24-.74.62-.9 1.07" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSunDim;
impl IconShape for LdSunDim {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="4" />
  <path d="M12 4h.01" />
  <path d="M20 12h.01" />
  <path d="M12 20h.01" />
  <path d="M4 12h.01" />
  <path d="M17.657 6.343h.01" />
  <path d="M17.657 17.657h.01" />
  <path d="M6.343 17.657h.01" />
  <path d="M6.343 6.343h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSunMedium;
impl IconShape for LdSunMedium {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="4" />
  <path d="M12 3v1" />
  <path d="M12 20v1" />
  <path d="M3 12h1" />
  <path d="M20 12h1" />
  <path d="m18.364 5.636-.707.707" />
  <path d="m6.343 17.657-.707.707" />
  <path d="m5.636 5.636.707.707" />
  <path d="m17.657 17.657.707.707" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSunMoon;
impl IconShape for LdSunMoon {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 8a2.83 2.83 0 0 0 4 4 4 4 0 1 1-4-4" />
  <path d="M12 2v2" />
  <path d="M12 20v2" />
  <path d="m4.9 4.9 1.4 1.4" />
  <path d="m17.7 17.7 1.4 1.4" />
  <path d="M2 12h2" />
  <path d="M20 12h2" />
  <path d="m6.3 17.7-1.4 1.4" />
  <path d="m19.1 4.9-1.4 1.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSunSnow;
impl IconShape for LdSunSnow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 9a3 3 0 1 0 0 6" />
  <path d="M2 12h1" />
  <path d="M14 21V3" />
  <path d="M10 4V3" />
  <path d="M10 21v-1" />
  <path d="m3.64 18.36.7-.7" />
  <path d="m4.34 6.34-.7-.7" />
  <path d="M14 12h8" />
  <path d="m17 4-3 3" />
  <path d="m14 17 3 3" />
  <path d="m21 15-3-3 3-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSun;
impl IconShape for LdSun {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="4" />
  <path d="M12 2v2" />
  <path d="M12 20v2" />
  <path d="m4.93 4.93 1.41 1.41" />
  <path d="m17.66 17.66 1.41 1.41" />
  <path d="M2 12h2" />
  <path d="M20 12h2" />
  <path d="m6.34 17.66-1.41 1.41" />
  <path d="m19.07 4.93-1.41 1.41" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSunrise;
impl IconShape for LdSunrise {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v8" />
  <path d="m4.93 10.93 1.41 1.41" />
  <path d="M2 18h2" />
  <path d="M20 18h2" />
  <path d="m19.07 10.93-1.41 1.41" />
  <path d="M22 22H2" />
  <path d="m8 6 4-4 4 4" />
  <path d="M16 18a4 4 0 0 0-8 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSunset;
impl IconShape for LdSunset {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 10V2" />
  <path d="m4.93 10.93 1.41 1.41" />
  <path d="M2 18h2" />
  <path d="M20 18h2" />
  <path d="m19.07 10.93-1.41 1.41" />
  <path d="M22 22H2" />
  <path d="m16 6-4 4-4-4" />
  <path d="M16 18a4 4 0 0 0-8 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSuperscript;
impl IconShape for LdSuperscript {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m4 19 8-8" />
  <path d="m12 19-8-8" />
  <path d="M20 12h-4c0-1.5.442-2 1.5-2.5S20 8.334 20 7.002c0-.472-.17-.93-.484-1.29a2.105 2.105 0 0 0-2.617-.436c-.42.239-.738.614-.899 1.06" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSwatchBook;
impl IconShape for LdSwatchBook {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 17a4 4 0 0 1-8 0V5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2Z" />
  <path d="M16.7 13H19a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H7" />
  <path d="M 7 17h0.01" />
  <path d="m11 8 2.3-2.3a2.4 2.4 0 0 1 3.404.004L18.6 7.6a2.4 2.4 0 0 1 .026 3.434L9.9 19.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSwissFranc;
impl IconShape for LdSwissFranc {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 21V3h8" />
  <path d="M6 16h9" />
  <path d="M10 9.5h7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSwitchCamera;
impl IconShape for LdSwitchCamera {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" />
  <path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" />
  <circle cx="12" cy="12" r="3" />
  <path d="m18 22-3-3 3-3" />
  <path d="m6 2 3 3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSword;
impl IconShape for LdSword {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5" />
  <line x1="13" x2="19" y1="19" y2="13" />
  <line x1="16" x2="20" y1="16" y2="20" />
  <line x1="19" x2="21" y1="21" y2="19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSwords;
impl IconShape for LdSwords {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="14.5 17.5 3 6 3 3 6 3 17.5 14.5" />
  <line x1="13" x2="19" y1="19" y2="13" />
  <line x1="16" x2="20" y1="16" y2="20" />
  <line x1="19" x2="21" y1="21" y2="19" />
  <polyline points="14.5 6.5 18 3 21 3 21 6 17.5 9.5" />
  <line x1="5" x2="9" y1="14" y2="18" />
  <line x1="7" x2="4" y1="17" y2="20" />
  <line x1="3" x2="5" y1="19" y2="21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdSyringe;
impl IconShape for LdSyringe {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18 2 4 4" />
  <path d="m17 7 3-3" />
  <path d="M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5" />
  <path d="m9 11 4 4" />
  <path d="m5 19-3 3" />
  <path d="m14 4 6 6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTable2;
impl IconShape for LdTable2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTableCellsMerge;
impl IconShape for LdTableCellsMerge {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 21v-6" />
  <path d="M12 9V3" />
  <path d="M3 15h18" />
  <path d="M3 9h18" />
  <rect width="18" height="18" x="3" y="3" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTableCellsSplit;
impl IconShape for LdTableCellsSplit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 15V9" />
  <path d="M3 15h18" />
  <path d="M3 9h18" />
  <rect width="18" height="18" x="3" y="3" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTableColumnsSplit;
impl IconShape for LdTableColumnsSplit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 14v2" />
  <path d="M14 20v2" />
  <path d="M14 2v2" />
  <path d="M14 8v2" />
  <path d="M2 15h8" />
  <path d="M2 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H2" />
  <path d="M2 9h8" />
  <path d="M22 15h-4" />
  <path d="M22 3h-2a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2" />
  <path d="M22 9h-4" />
  <path d="M5 3v18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTableProperties;
impl IconShape for LdTableProperties {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 3v18" />
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M21 9H3" />
  <path d="M21 15H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTableRowsSplit;
impl IconShape for LdTableRowsSplit {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 10h2" />
  <path d="M15 22v-8" />
  <path d="M15 2v4" />
  <path d="M2 10h2" />
  <path d="M20 10h2" />
  <path d="M3 19h18" />
  <path d="M3 22v-6a2 2 135 0 1 2-2h14a2 2 45 0 1 2 2v6" />
  <path d="M3 2v2a2 2 45 0 0 2 2h14a2 2 135 0 0 2-2V2" />
  <path d="M8 10h2" />
  <path d="M9 22v-8" />
  <path d="M9 2v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTable;
impl IconShape for LdTable {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 3v18" />
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9h18" />
  <path d="M3 15h18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTabletSmartphone;
impl IconShape for LdTabletSmartphone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="10" height="14" x="3" y="8" rx="2" />
  <path d="M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4" />
  <path d="M8 18h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTablet;
impl IconShape for LdTablet {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect  height="20" x="4" y="2" rx="2" ry="2" />
  <line x1="12" x2="12.01" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTablets;
impl IconShape for LdTablets {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="7" cy="7" r="5" />
  <circle cx="17" cy="17" r="5" />
  <path d="M12 17h10" />
  <path d="m3.46 10.54 7.08-7.08" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTag;
impl IconShape for LdTag {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z" />
  <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTags;
impl IconShape for LdTags {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m15 5 6.3 6.3a2.4 2.4 0 0 1 0 3.4L17 19" />
  <path d="M9.586 5.586A2 2 0 0 0 8.172 5H3a1 1 0 0 0-1 1v5.172a2 2 0 0 0 .586 1.414L8.29 18.29a2.426 2.426 0 0 0 3.42 0l3.58-3.58a2.426 2.426 0 0 0 0-3.42z" />
  <circle cx="6.5" cy="9.5" r=".5" fill="currentColor" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTally1;
impl IconShape for LdTally1 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTally2;
impl IconShape for LdTally2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4v16" />
  <path d="M9 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTally3;
impl IconShape for LdTally3 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4v16" />
  <path d="M9 4v16" />
  <path d="M14 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTally4;
impl IconShape for LdTally4 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4v16" />
  <path d="M9 4v16" />
  <path d="M14 4v16" />
  <path d="M19 4v16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTally5;
impl IconShape for LdTally5 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4v16" />
  <path d="M9 4v16" />
  <path d="M14 4v16" />
  <path d="M19 4v16" />
  <path d="M22 6 2 18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTangent;
impl IconShape for LdTangent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="17" cy="4" r="2" />
  <path d="M15.59 5.41 5.41 15.59" />
  <circle cx="4" cy="17" r="2" />
  <path d="M12 22s-4-9-1.5-11.5S22 12 22 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTarget;
impl IconShape for LdTarget {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="10" />
  <circle cx="12" cy="12" r="6" />
  <circle cx="12" cy="12" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTelescope;
impl IconShape for LdTelescope {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m10.065 12.493-6.18 1.318a.934.934 0 0 1-1.108-.702l-.537-2.15a1.07 1.07 0 0 1 .691-1.265l13.504-4.44" />
  <path d="m13.56 11.747 4.332-.924" />
  <path d="m16 21-3.105-6.21" />
  <path d="M16.485 5.94a2 2 0 0 1 1.455-2.425l1.09-.272a1 1 0 0 1 1.212.727l1.515 6.06a1 1 0 0 1-.727 1.213l-1.09.272a2 2 0 0 1-2.425-1.455z" />
  <path d="m6.158 8.633 1.114 4.456" />
  <path d="m8 21 3.105-6.21" />
  <circle cx="12" cy="13" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTentTree;
impl IconShape for LdTentTree {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="4" cy="4" r="2" />
  <path d="m14 5 3-3 3 3" />
  <path d="m14 10 3-3 3 3" />
  <path d="M17 14V2" />
  <path d="M17 14H7l-5 8h20Z" />
  <path d="M8 14v8" />
  <path d="m9 14 5 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTent;
impl IconShape for LdTent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3.5 21 14 3" />
  <path d="M20.5 21 10 3" />
  <path d="M15.5 21 12 15l-3.5 6" />
  <path d="M2 21h20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTerminal;
impl IconShape for LdTerminal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="4 17 10 11 4 5" />
  <line x1="12" x2="20" y1="19" y2="19" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTestTubeDiagonal;
impl IconShape for LdTestTubeDiagonal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 7 6.82 21.18a2.83 2.83 0 0 1-3.99-.01v0a2.83 2.83 0 0 1 0-4L17 3" />
  <path d="m16 2 6 6" />
  <path d="M12 16H4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTestTube;
impl IconShape for LdTestTube {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5h0c-1.4 0-2.5-1.1-2.5-2.5V2" />
  <path d="M8.5 2h7" />
  <path d="M14.5 16h-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTestTubes;
impl IconShape for LdTestTubes {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 2v17.5A2.5 2.5 0 0 1 6.5 22v0A2.5 2.5 0 0 1 4 19.5V2" />
  <path d="M20 2v17.5a2.5 2.5 0 0 1-2.5 2.5v0a2.5 2.5 0 0 1-2.5-2.5V2" />
  <path d="M3 2h7" />
  <path d="M14 2h7" />
  <path d="M9 16H4" />
  <path d="M20 16h-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTextCursorInput;
impl IconShape for LdTextCursorInput {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 4h1a3 3 0 0 1 3 3 3 3 0 0 1 3-3h1" />
  <path d="M13 20h-1a3 3 0 0 1-3-3 3 3 0 0 1-3 3H5" />
  <path d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1" />
  <path d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7" />
  <path d="M9 7v10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTextCursor;
impl IconShape for LdTextCursor {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1" />
  <path d="M7 22h1a4 4 0 0 0 4-4v-1" />
  <path d="M7 2h1a4 4 0 0 1 4 4v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTextQuote;
impl IconShape for LdTextQuote {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 6H3" />
  <path d="M21 12H8" />
  <path d="M21 18H8" />
  <path d="M3 12v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTextSearch;
impl IconShape for LdTextSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 6H3" />
  <path d="M10 12H3" />
  <path d="M10 18H3" />
  <circle cx="17" cy="15" r="3" />
  <path d="m21 19-1.9-1.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTextSelect;
impl IconShape for LdTextSelect {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 3a2 2 0 0 0-2 2" />
  <path d="M19 3a2 2 0 0 1 2 2" />
  <path d="M21 19a2 2 0 0 1-2 2" />
  <path d="M5 21a2 2 0 0 1-2-2" />
  <path d="M9 3h1" />
  <path d="M9 21h1" />
  <path d="M14 3h1" />
  <path d="M14 21h1" />
  <path d="M3 9v1" />
  <path d="M21 9v1" />
  <path d="M3 14v1" />
  <path d="M21 14v1" />
  <line x1="7" x2="15" y1="8" y2="8" />
  <line x1="7" x2="17" y1="12" y2="12" />
  <line x1="7" x2="13" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdText;
impl IconShape for LdText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 6.1H3" />
  <path d="M21 12.1H3" />
  <path d="M15.1 18H3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTheater;
impl IconShape for LdTheater {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 10s3-3 3-8" />
  <path d="M22 10s-3-3-3-8" />
  <path d="M10 2c0 4.4-3.6 8-8 8" />
  <path d="M14 2c0 4.4 3.6 8 8 8" />
  <path d="M2 10s2 2 2 5" />
  <path d="M22 10s-2 2-2 5" />
  <path d="M8 15h8" />
  <path d="M2 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1" />
  <path d="M14 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdThermometerSnowflake;
impl IconShape for LdThermometerSnowflake {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12h10" />
  <path d="M9 4v16" />
  <path d="m3 9 3 3-3 3" />
  <path d="M12 6 9 9 6 6" />
  <path d="m6 18 3-3 1.5 1.5" />
  <path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdThermometerSun;
impl IconShape for LdThermometerSun {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 9a4 4 0 0 0-2 7.5" />
  <path d="M12 3v2" />
  <path d="m6.6 18.4-1.4 1.4" />
  <path d="M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" />
  <path d="M4 13H2" />
  <path d="M6.34 7.34 4.93 5.93" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdThermometer;
impl IconShape for LdThermometer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdThumbsDown;
impl IconShape for LdThumbsDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 14V2" />
  <path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdThumbsUp;
impl IconShape for LdThumbsUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 10v12" />
  <path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicketCheck;
impl IconShape for LdTicketCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="m9 12 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicketMinus;
impl IconShape for LdTicketMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="M9 12h6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicketPercent;
impl IconShape for LdTicketPercent {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 1 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 1 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="M9 9h.01" />
  <path d="m15 9-6 6" />
  <path d="M15 15h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicketPlus;
impl IconShape for LdTicketPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="M9 12h6" />
  <path d="M12 9v6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicketSlash;
impl IconShape for LdTicketSlash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="m9.5 14.5 5-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicketX;
impl IconShape for LdTicketX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="m9.5 14.5 5-5" />
  <path d="m9.5 9.5 5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTicket;
impl IconShape for LdTicket {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z" />
  <path d="M13 5v2" />
  <path d="M13 17v2" />
  <path d="M13 11v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTimerOff;
impl IconShape for LdTimerOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 2h4" />
  <path d="M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7" />
  <path d="M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2" />
  <path d="m2 2 20 20" />
  <path d="M12 12v-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTimerReset;
impl IconShape for LdTimerReset {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 2h4" />
  <path d="M12 14v-4" />
  <path d="M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6" />
  <path d="M9 17H4v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTimer;
impl IconShape for LdTimer {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="10" x2="14" y1="2" y2="2" />
  <line x1="12" x2="15" y1="14" y2="11" />
  <circle cx="12" cy="14" r="8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdToggleLeft;
impl IconShape for LdToggleLeft {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="12" x="2" y="6" rx="6" ry="6" />
  <circle cx="8" cy="12" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdToggleRight;
impl IconShape for LdToggleRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="12" x="2" y="6" rx="6" ry="6" />
  <circle cx="16" cy="12" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTornado;
impl IconShape for LdTornado {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 4H3" />
  <path d="M18 8H6" />
  <path d="M19 12H9" />
  <path d="M16 16h-6" />
  <path d="M11 20H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTorus;
impl IconShape for LdTorus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <ellipse cx="12" cy="11" rx="3" ry="2" />
  <ellipse cx="12" cy="12.5" rx="10" ry="8.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTouchpadOff;
impl IconShape for LdTouchpadOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16" />
  <path d="M2 14h12" />
  <path d="M22 14h-2" />
  <path d="M12 20v-6" />
  <path d="m2 2 20 20" />
  <path d="M22 16V6a2 2 0 0 0-2-2H10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTouchpad;
impl IconShape for LdTouchpad {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="M2 14h20" />
  <path d="M12 20v-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTowerControl;
impl IconShape for LdTowerControl {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z" />
  <path d="M8 13v9" />
  <path d="M16 22v-9" />
  <path d="m9 6 1 7" />
  <path d="m15 6-1 7" />
  <path d="M12 6V2" />
  <path d="M13 2h-2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdToyBrick;
impl IconShape for LdToyBrick {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="12" x="3" y="8" rx="1" />
  <path d="M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3" />
  <path d="M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTractor;
impl IconShape for LdTractor {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m10 11 11 .9c.6 0 .9.5.8 1.1l-.8 5h-1" />
  <path d="M16 18h-5" />
  <path d="M18 5a1 1 0 0 0-1 1v5.573" />
  <path d="M3 4h9l1 7.246" />
  <path d="M4 11V4" />
  <path d="M7 15h.01" />
  <path d="M8 10.1V4" />
  <circle cx="18" cy="18" r="2" />
  <circle cx="7" cy="15" r="5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrafficCone;
impl IconShape for LdTrafficCone {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9.3 6.2a4.55 4.55 0 0 0 5.4 0" />
  <path d="M7.9 10.7c.9.8 2.4 1.3 4.1 1.3s3.2-.5 4.1-1.3" />
  <path d="M13.9 3.5a1.93 1.93 0 0 0-3.8-.1l-3 10c-.1.2-.1.4-.1.6 0 1.7 2.2 3 5 3s5-1.3 5-3c0-.2 0-.4-.1-.5Z" />
  <path d="m7.5 12.2-4.7 2.7c-.5.3-.8.7-.8 1.1s.3.8.8 1.1l7.6 4.5c.9.5 2.1.5 3 0l7.6-4.5c.7-.3 1-.7 1-1.1s-.3-.8-.8-1.1l-4.7-2.8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrainFrontTunnel;
impl IconShape for LdTrainFrontTunnel {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 22V12a10 10 0 1 1 20 0v10" />
  <path d="M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8" />
  <path d="M10 15h.01" />
  <path d="M14 15h.01" />
  <path d="M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z" />
  <path d="m9 19-2 3" />
  <path d="m15 19 2 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrainFront;
impl IconShape for LdTrainFront {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 3.1V7a4 4 0 0 0 8 0V3.1" />
  <path d="m9 15-1-1" />
  <path d="m15 15 1-1" />
  <path d="M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z" />
  <path d="m8 19-2 3" />
  <path d="m16 19 2 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrainTrack;
impl IconShape for LdTrainTrack {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 17 17 2" />
  <path d="m2 14 8 8" />
  <path d="m5 11 8 8" />
  <path d="m8 8 8 8" />
  <path d="m11 5 8 8" />
  <path d="m14 2 8 8" />
  <path d="M7 22 22 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTramFront;
impl IconShape for LdTramFront {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect   x="4" y="3" rx="2" />
  <path d="M4 11h16" />
  <path d="M12 3v8" />
  <path d="m8 19-2 3" />
  <path d="m18 22-2-3" />
  <path d="M8 15h.01" />
  <path d="M16 15h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrash2;
impl IconShape for LdTrash2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 6h18" />
  <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
  <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
  <line x1="10" x2="10" y1="11" y2="17" />
  <line x1="14" x2="14" y1="11" y2="17" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrash;
impl IconShape for LdTrash {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 6h18" />
  <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
  <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTreeDeciduous;
impl IconShape for LdTreeDeciduous {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 19a4 4 0 0 1-2.24-7.32A3.5 3.5 0 0 1 9 6.03V6a3 3 0 1 1 6 0v.04a3.5 3.5 0 0 1 3.24 5.65A4 4 0 0 1 16 19Z" />
  <path d="M12 19v3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTreePalm;
impl IconShape for LdTreePalm {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13 8c0-2.76-2.46-5-5.5-5S2 5.24 2 8h2l1-1 1 1h4" />
  <path d="M13 7.14A5.82 5.82 0 0 1 16.5 6c3.04 0 5.5 2.24 5.5 5h-3l-1-1-1 1h-3" />
  <path d="M5.89 9.71c-2.15 2.15-2.3 5.47-.35 7.43l4.24-4.25.7-.7.71-.71 2.12-2.12c-1.95-1.96-5.27-1.8-7.42.35" />
  <path d="M11 15.5c.5 2.5-.17 4.5-1 6.5h4c2-5.5-.5-12-1-14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTreePine;
impl IconShape for LdTreePine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z" />
  <path d="M12 22v-3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrees;
impl IconShape for LdTrees {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10 10v.2A3 3 0 0 1 8.9 16v0H5v0h0a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z" />
  <path d="M7 16v6" />
  <path d="M13 19v3" />
  <path d="M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrello;
impl IconShape for LdTrello {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
  <rect width="3" height="9" x="7" y="7" />
  <rect width="3" height="5" x="14" y="7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrendingDown;
impl IconShape for LdTrendingDown {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="22 17 13.5 8.5 8.5 13.5 2 7" />
  <polyline points="16 17 22 17 22 11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrendingUp;
impl IconShape for LdTrendingUp {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="22 7 13.5 15.5 8.5 10.5 2 17" />
  <polyline points="16 7 22 7 22 13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTriangleAlert;
impl IconShape for LdTriangleAlert {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" />
  <path d="M12 9v4" />
  <path d="M12 17h.01" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTriangleRight;
impl IconShape for LdTriangleRight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 18a2 2 0 0 1-2 2H3c-1.1 0-1.3-.6-.4-1.3L20.4 4.3c.9-.7 1.6-.4 1.6.7Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTriangle;
impl IconShape for LdTriangle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M13.73 4a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTrophy;
impl IconShape for LdTrophy {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6" />
  <path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18" />
  <path d="M4 22h16" />
  <path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22" />
  <path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22" />
  <path d="M18 2H6v7a6 6 0 0 0 12 0V2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTruck;
impl IconShape for LdTruck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2" />
  <path d="M15 18H9" />
  <path d="M19 18h2a1 1 0 0 0 1-1v-3.65a1 1 0 0 0-.22-.624l-3.48-4.35A1 1 0 0 0 17.52 8H14" />
  <circle cx="17" cy="18" r="2" />
  <circle cx="7" cy="18" r="2" />
</svg>"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTurtle;
impl IconShape for LdTurtle {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z" />
  <path d="M4.82 7.9 8 10" />
  <path d="M15.18 7.9 12 10" />
  <path d="M16.93 10H20a2 2 0 0 1 0 4H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTv2;
impl IconShape for LdTv2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M7 21h10" />
  <rect width="20" height="14" x="2" y="3" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTv;
impl IconShape for LdTv {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20" height="15" x="2" y="7" rx="2" ry="2" />
  <polyline points="17 2 12 7 7 2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTwitch;
impl IconShape for LdTwitch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdTwitter;
impl IconShape for LdTwitter {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdType;
impl IconShape for LdType {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polyline points="4 7 4 4 20 4 20 7" />
  <line x1="9" x2="15" y1="20" y2="20" />
  <line x1="12" x2="12" y1="4" y2="20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUmbrellaOff;
impl IconShape for LdUmbrellaOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v1" />
  <path d="M15.5 21a1.85 1.85 0 0 1-3.5-1v-8H2a10 10 0 0 1 3.428-6.575" />
  <path d="M17.5 12H22A10 10 0 0 0 9.004 3.455" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUmbrella;
impl IconShape for LdUmbrella {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 12a10.06 10.06 1 0 0-20 0Z" />
  <path d="M12 12v8a2 2 0 0 0 4 0" />
  <path d="M12 2v1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUnderline;
impl IconShape for LdUnderline {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M6 4v6a6 6 0 0 0 12 0V4" />
  <line x1="4" x2="20" y1="20" y2="20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUndo2;
impl IconShape for LdUndo2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M9 14 4 9l5-5" />
  <path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5v0a5.5 5.5 0 0 1-5.5 5.5H11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUndoDot;
impl IconShape for LdUndoDot {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="17" r="1" />
  <path d="M3 7v6h6" />
  <path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUndo;
impl IconShape for LdUndo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 7v6h6" />
  <path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUnfoldHorizontal;
impl IconShape for LdUnfoldHorizontal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 12h6" />
  <path d="M8 12H2" />
  <path d="M12 2v2" />
  <path d="M12 8v2" />
  <path d="M12 14v2" />
  <path d="M12 20v2" />
  <path d="m19 15 3-3-3-3" />
  <path d="m5 9-3 3 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUnfoldVertical;
impl IconShape for LdUnfoldVertical {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 22v-6" />
  <path d="M12 8V2" />
  <path d="M4 12H2" />
  <path d="M10 12H8" />
  <path d="M16 12h-2" />
  <path d="M22 12h-2" />
  <path d="m15 19-3 3-3-3" />
  <path d="m15 5-3-3-3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUngroup;
impl IconShape for LdUngroup {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="6" x="5" y="4" rx="1" />
  <rect width="8" height="6" x="11" y="14" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUniversity;
impl IconShape for LdUniversity {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="10" r="1" />
  <path d="M22 20V8h-4l-6-4-6 4H2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2" />
  <path d="M6 17v.01" />
  <path d="M6 13v.01" />
  <path d="M18 17v.01" />
  <path d="M18 13v.01" />
  <path d="M14 22v-5a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUnlink2;
impl IconShape for LdUnlink2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUnlink;
impl IconShape for LdUnlink {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71" />
  <path d="m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71" />
  <line x1="8" x2="8" y1="2" y2="5" />
  <line x1="2" x2="5" y1="8" y2="8" />
  <line x1="16" x2="16" y1="19" y2="22" />
  <line x1="19" x2="22" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUnplug;
impl IconShape for LdUnplug {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 5 3-3" />
  <path d="m2 22 3-3" />
  <path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z" />
  <path d="M7.5 13.5 10 11" />
  <path d="M10.5 16.5 13 14" />
  <path d="m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUpload;
impl IconShape for LdUpload {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
  <polyline points="17 8 12 3 7 8" />
  <line x1="12" x2="12" y1="3" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUsb;
impl IconShape for LdUsb {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="10" cy="7" r="1" />
  <circle cx="4" cy="20" r="1" />
  <path d="M4.7 19.3 19 5" />
  <path d="m21 3-3 1 2 2Z" />
  <path d="M9.26 7.68 5 12l2 5" />
  <path d="m10 14 5 2 3.5-3.5" />
  <path d="m18 12 1-1 1 1-1 1Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserCheck;
impl IconShape for LdUserCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
  <circle cx="9" cy="7" r="4" />
  <polyline points="16 11 18 13 22 9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserCog;
impl IconShape for LdUserCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="18" cy="15" r="3" />
  <circle cx="9" cy="7" r="4" />
  <path d="M10 15H6a4 4 0 0 0-4 4v2" />
  <path d="m21.7 16.4-.9-.3" />
  <path d="m15.2 13.9-.9-.3" />
  <path d="m16.6 18.7.3-.9" />
  <path d="m19.1 12.2.3-.9" />
  <path d="m19.6 18.7-.4-1" />
  <path d="m16.8 12.3-.4-1" />
  <path d="m14.3 16.6 1-.4" />
  <path d="m20.7 13.8 1-.4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserMinus;
impl IconShape for LdUserMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
  <circle cx="9" cy="7" r="4" />
  <line x1="22" x2="16" y1="11" y2="11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserPlus;
impl IconShape for LdUserPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
  <circle cx="9" cy="7" r="4" />
  <line x1="19" x2="19" y1="8" y2="14" />
  <line x1="22" x2="16" y1="11" y2="11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRoundCheck;
impl IconShape for LdUserRoundCheck {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 21a8 8 0 0 1 13.292-6" />
  <circle cx="10" cy="8" r="5" />
  <path d="m16 19 2 2 4-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRoundCog;
impl IconShape for LdUserRoundCog {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 21a8 8 0 0 1 10.434-7.62" />
  <circle cx="10" cy="8" r="5" />
  <circle cx="18" cy="18" r="3" />
  <path d="m19.5 14.3-.4.9" />
  <path d="m16.9 20.8-.4.9" />
  <path d="m21.7 19.5-.9-.4" />
  <path d="m15.2 16.9-.9-.4" />
  <path d="m21.7 16.5-.9.4" />
  <path d="m15.2 19.1-.9.4" />
  <path d="m19.5 21.7-.4-.9" />
  <path d="m16.9 15.2-.4-.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRoundMinus;
impl IconShape for LdUserRoundMinus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 21a8 8 0 0 1 13.292-6" />
  <circle cx="10" cy="8" r="5" />
  <path d="M22 19h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRoundPlus;
impl IconShape for LdUserRoundPlus {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 21a8 8 0 0 1 13.292-6" />
  <circle cx="10" cy="8" r="5" />
  <path d="M19 16v6" />
  <path d="M22 19h-6" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRoundSearch;
impl IconShape for LdUserRoundSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="10" cy="8" r="5" />
  <path d="M2 21a8 8 0 0 1 10.434-7.62" />
  <circle cx="18" cy="18" r="3" />
  <path d="m22 22-1.9-1.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRoundX;
impl IconShape for LdUserRoundX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 21a8 8 0 0 1 11.873-7" />
  <circle cx="10" cy="8" r="5" />
  <path d="m17 17 5 5" />
  <path d="m22 17-5 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserRound;
impl IconShape for LdUserRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="8" r="5" />
  <path d="M20 21a8 8 0 0 0-16 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserSearch;
impl IconShape for LdUserSearch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="10" cy="7" r="4" />
  <path d="M10.3 15H7a4 4 0 0 0-4 4v2" />
  <circle cx="17" cy="17" r="3" />
  <path d="m21 21-1.9-1.9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUserX;
impl IconShape for LdUserX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
  <circle cx="9" cy="7" r="4" />
  <line x1="17" x2="22" y1="8" y2="13" />
  <line x1="22" x2="17" y1="8" y2="13" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUser;
impl IconShape for LdUser {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
  <circle cx="12" cy="7" r="4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUsersRound;
impl IconShape for LdUsersRound {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 21a8 8 0 0 0-16 0" />
  <circle cx="10" cy="8" r="5" />
  <path d="M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUsers;
impl IconShape for LdUsers {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
  <circle cx="9" cy="7" r="4" />
  <path d="M22 21v-2a4 4 0 0 0-3-3.87" />
  <path d="M16 3.13a4 4 0 0 1 0 7.75" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUtensilsCrossed;
impl IconShape for LdUtensilsCrossed {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 2-2.3 2.3a3 3 0 0 0 0 4.2l1.8 1.8a3 3 0 0 0 4.2 0L22 8" />
  <path d="M15 15 3.3 3.3a4.2 4.2 0 0 0 0 6l7.3 7.3c.7.7 2 .7 2.8 0L15 15Zm0 0 7 7" />
  <path d="m2.1 21.8 6.4-6.3" />
  <path d="m19 5-7 7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUtensils;
impl IconShape for LdUtensils {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2" />
  <path d="M7 2v20" />
  <path d="M21 15V2v0a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdUtilityPole;
impl IconShape for LdUtilityPole {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 2v20" />
  <path d="M2 5h20" />
  <path d="M3 3v2" />
  <path d="M7 3v2" />
  <path d="M17 3v2" />
  <path d="M21 3v2" />
  <path d="m19 5-7 7-7-7" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVariable;
impl IconShape for LdVariable {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 21s-4-3-4-9 4-9 4-9" />
  <path d="M16 3s4 3 4 9-4 9-4 9" />
  <line x1="15" x2="9" y1="9" y2="15" />
  <line x1="9" x2="15" y1="9" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVault;
impl IconShape for LdVault {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
  <path d="m7.9 7.9 2.7 2.7" />
  <circle cx="16.5" cy="7.5" r=".5" fill="currentColor" />
  <path d="m13.4 10.6 2.7-2.7" />
  <circle cx="7.5" cy="16.5" r=".5" fill="currentColor" />
  <path d="m7.9 16.1 2.7-2.7" />
  <circle cx="16.5" cy="16.5" r=".5" fill="currentColor" />
  <path d="m13.4 13.4 2.7 2.7" />
  <circle cx="12" cy="12" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVegan;
impl IconShape for LdVegan {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14" />
  <path d="M16 8c4 0 6-2 6-6-4 0-6 2-6 6" />
  <path d="M17.41 3.6a10 10 0 1 0 3 3" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVenetianMask;
impl IconShape for LdVenetianMask {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 12a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V7h-5a8 8 0 0 0-5 2 8 8 0 0 0-5-2H2Z" />
  <path d="M6 11c1.5 0 3 .5 3 2-2 0-3 0-3-2Z" />
  <path d="M18 11c-1.5 0-3 .5-3 2 2 0 3 0 3-2Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVibrateOff;
impl IconShape for LdVibrateOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 8 2 2-2 2 2 2-2 2" />
  <path d="m22 8-2 2 2 2-2 2 2 2" />
  <path d="M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2" />
  <path d="M16 10.34V6c0-.55-.45-1-1-1h-4.34" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVibrate;
impl IconShape for LdVibrate {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 8 2 2-2 2 2 2-2 2" />
  <path d="m22 8-2 2 2 2-2 2 2 2" />
  <rect width="8" height="14" x="8" y="5" rx="1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVideoOff;
impl IconShape for LdVideoOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.66 6H14a2 2 0 0 1 2 2v2.5l5.248-3.062A.5.5 0 0 1 22 7.87v8.196" />
  <path d="M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVideo;
impl IconShape for LdVideo {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5" />
  <rect x="2" y="6" width="14" height="12" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVideotape;
impl IconShape for LdVideotape {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="20"  x="2" y="4" rx="2" />
  <path d="M2 8h20" />
  <circle cx="8" cy="14" r="2" />
  <path d="M8 12h8" />
  <circle cx="16" cy="14" r="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdView;
impl IconShape for LdView {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M5 12s2.545-5 7-5c4.454 0 7 5 7 5s-2.546 5-7 5c-4.455 0-7-5-7-5z" />
  <path d="M12 13a1 1 0 1 0 0-2 1 1 0 0 0 0 2z" />
  <path d="M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2" />
  <path d="M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVoicemail;
impl IconShape for LdVoicemail {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="6" cy="12" r="4" />
  <circle cx="18" cy="12" r="4" />
  <line x1="6" x2="18" y1="16" y2="16" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVolume1;
impl IconShape for LdVolume1 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
  <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVolume2;
impl IconShape for LdVolume2 {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
  <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
  <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVolumeX;
impl IconShape for LdVolumeX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
  <line x1="22" x2="16" y1="9" y2="15" />
  <line x1="16" x2="22" y1="9" y2="15" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVolume;
impl IconShape for LdVolume {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdVote;
impl IconShape for LdVote {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m9 12 2 2 4-4" />
  <path d="M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z" />
  <path d="M22 19H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWalletCards;
impl IconShape for LdWalletCards {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="18" height="18" x="3" y="3" rx="2" />
  <path d="M3 9a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2" />
  <path d="M3 11h3c.8 0 1.6.3 2.1.9l1.1.9c1.6 1.6 4.1 1.6 5.7 0l1.1-.9c.5-.5 1.3-.9 2.1-.9H21" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWalletMinimal;
impl IconShape for LdWalletMinimal {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 14h.01" />
  <path d="M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWallet;
impl IconShape for LdWallet {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" />
  <path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWallpaper;
impl IconShape for LdWallpaper {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="8" cy="9" r="2" />
  <path d="m9 17 6.1-6.1a2 2 0 0 1 2.81.01L22 15V5a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2" />
  <path d="M8 21h8" />
  <path d="M12 17v4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWandSparkles;
impl IconShape for LdWandSparkles {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72" />
  <path d="m14 7 3 3" />
  <path d="M5 6v4" />
  <path d="M19 14v4" />
  <path d="M10 2v2" />
  <path d="M7 8H3" />
  <path d="M21 16h-4" />
  <path d="M11 3H9" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWand;
impl IconShape for LdWand {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M15 4V2" />
  <path d="M15 16v-2" />
  <path d="M8 9h2" />
  <path d="M20 9h2" />
  <path d="M17.8 11.8 19 13" />
  <path d="M15 9h0" />
  <path d="M17.8 6.2 19 5" />
  <path d="m3 21 9-9" />
  <path d="M12.2 6.2 11 5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWarehouse;
impl IconShape for LdWarehouse {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M22 8.35V20a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8.35A2 2 0 0 1 3.26 6.5l8-3.2a2 2 0 0 1 1.48 0l8 3.2A2 2 0 0 1 22 8.35Z" />
  <path d="M6 18h12" />
  <path d="M6 14h12" />
  <rect width="12" height="12" x="6" y="10" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWashingMachine;
impl IconShape for LdWashingMachine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M3 6h3" />
  <path d="M17 6h.01" />
  <rect width="18" height="20" x="3" y="2" rx="2" />
  <circle cx="12" cy="13" r="5" />
  <path d="M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWatch;
impl IconShape for LdWatch {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="12" r="6" />
  <polyline points="12 10 12 12 13 13" />
  <path d="m16.13 7.66-.81-4.05a2 2 0 0 0-2-1.61h-2.68a2 2 0 0 0-2 1.61l-.78 4.05" />
  <path d="m7.88 16.36.8 4a2 2 0 0 0 2 1.61h2.72a2 2 0 0 0 2-1.61l.81-4.05" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWaves;
impl IconShape for LdWaves {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 6c.6.5 1.2 1 2.5 1C7 7 7 5 9.5 5c2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
  <path d="M2 12c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
  <path d="M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWaypoints;
impl IconShape for LdWaypoints {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="4.5" r="2.5" />
  <path d="m10.2 6.3-3.9 3.9" />
  <circle cx="4.5" cy="12" r="2.5" />
  <path d="M7 12h10" />
  <circle cx="19.5" cy="12" r="2.5" />
  <path d="m13.8 17.7 3.9-3.9" />
  <circle cx="12" cy="19.5" r="2.5" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWebcam;
impl IconShape for LdWebcam {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="10" r="8" />
  <circle cx="12" cy="10" r="3" />
  <path d="M7 22h10" />
  <path d="M12 22v-4" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWebhookOff;
impl IconShape for LdWebhookOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17 17h-5c-1.09-.02-1.94.92-2.5 1.9A3 3 0 1 1 2.57 15" />
  <path d="M9 3.4a4 4 0 0 1 6.52.66" />
  <path d="m6 17 3.1-5.8a2.5 2.5 0 0 0 .057-2.05" />
  <path d="M20.3 20.3a4 4 0 0 1-2.3.7" />
  <path d="M18.6 13a4 4 0 0 1 3.357 3.414" />
  <path d="m12 6 .6 1" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWebhook;
impl IconShape for LdWebhook {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 16.98h-5.99c-1.1 0-1.95.94-2.48 1.9A4 4 0 0 1 2 17c.01-.7.2-1.4.57-2" />
  <path d="m6 17 3.13-5.78c.53-.97.1-2.18-.5-3.1a4 4 0 1 1 6.89-4.06" />
  <path d="m12 6 3.13 5.73C15.66 12.7 16.9 13 18 13a4 4 0 0 1 0 8" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWeight;
impl IconShape for LdWeight {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="12" cy="5" r="3" />
  <path d="M6.5 8a2 2 0 0 0-1.905 1.46L2.1 18.5A2 2 0 0 0 4 21h16a2 2 0 0 0 1.925-2.54L19.4 9.5A2 2 0 0 0 17.48 8Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWheatOff;
impl IconShape for LdWheatOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m2 22 10-10" />
  <path d="m16 8-1.17 1.17" />
  <path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
  <path d="m8 8-.53.53a3.5 3.5 0 0 0 0 4.94L9 15l1.53-1.53c.55-.55.88-1.25.98-1.97" />
  <path d="M10.91 5.26c.15-.26.34-.51.56-.73L13 3l1.53 1.53a3.5 3.5 0 0 1 .28 4.62" />
  <path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z" />
  <path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
  <path d="m16 16-.53.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.49 3.49 0 0 1 1.97-.98" />
  <path d="M18.74 13.09c.26-.15.51-.34.73-.56L21 11l-1.53-1.53a3.5 3.5 0 0 0-4.62-.28" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWheat;
impl IconShape for LdWheat {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2 22 16 8" />
  <path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
  <path d="M7.47 8.53 9 7l1.53 1.53a3.5 3.5 0 0 1 0 4.94L9 15l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
  <path d="M11.47 4.53 13 3l1.53 1.53a3.5 3.5 0 0 1 0 4.94L13 11l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z" />
  <path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z" />
  <path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
  <path d="M15.47 13.47 17 15l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
  <path d="M19.47 9.47 21 11l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L13 11l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWholeWord;
impl IconShape for LdWholeWord {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="7" cy="12" r="3" />
  <path d="M10 9v6" />
  <circle cx="17" cy="12" r="3" />
  <path d="M14 7v8" />
  <path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWifiOff;
impl IconShape for LdWifiOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 20h.01" />
  <path d="M8.5 16.429a5 5 0 0 1 7 0" />
  <path d="M5 12.859a10 10 0 0 1 5.17-2.69" />
  <path d="M19 12.859a10 10 0 0 0-2.007-1.523" />
  <path d="M2 8.82a15 15 0 0 1 4.177-2.643" />
  <path d="M22 8.82a15 15 0 0 0-11.288-3.764" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWifi;
impl IconShape for LdWifi {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M12 20h.01" />
  <path d="M2 8.82a15 15 0 0 1 20 0" />
  <path d="M5 12.859a10 10 0 0 1 14 0" />
  <path d="M8.5 16.429a5 5 0 0 1 7 0" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWind;
impl IconShape for LdWind {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M17.7 7.7a2.5 2.5 0 1 1 1.8 4.3H2" />
  <path d="M9.6 4.6A2 2 0 1 1 11 8H2" />
  <path d="M12.6 19.4A2 2 0 1 0 14 16H2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWineOff;
impl IconShape for LdWineOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 22h8" />
  <path d="M7 10h3m7 0h-1.343" />
  <path d="M12 15v7" />
  <path d="M7.307 7.307A12.33 12.33 0 0 0 7 10a5 5 0 0 0 7.391 4.391M8.638 2.981C8.75 2.668 8.872 2.34 9 2h6c1.5 4 2 6 2 8 0 .407-.05.809-.145 1.198" />
  <line x1="2" x2="22" y1="2" y2="22" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWine;
impl IconShape for LdWine {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M8 22h8" />
  <path d="M7 10h10" />
  <path d="M12 15v7" />
  <path d="M12 15a5 5 0 0 0 5-5c0-2-.5-4-2-8H9c-1.5 4-2 6-2 8a5 5 0 0 0 5 5Z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWorkflow;
impl IconShape for LdWorkflow {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <rect width="8" height="8" x="3" y="3" rx="2" />
  <path d="M7 11v4a2 2 0 0 0 2 2h4" />
  <rect width="8" height="8" x="13" y="13" rx="2" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWorm;
impl IconShape for LdWorm {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="m19 12-1.5 3" />
  <path d="M19.63 18.81 22 20" />
  <path d="M6.47 8.23a1.68 1.68 0 0 1 2.44 1.93l-.64 2.08a6.76 6.76 0 0 0 10.16 7.67l.42-.27a1 1 0 1 0-2.73-4.21l-.42.27a1.76 1.76 0 0 1-2.63-1.99l.64-2.08A6.66 6.66 0 0 0 3.94 3.9l-.7.4a1 1 0 1 0 2.55 4.34z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWrapText;
impl IconShape for LdWrapText {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <line x1="3" x2="21" y1="6" y2="6" />
  <path d="M3 12h15a3 3 0 1 1 0 6h-4" />
  <polyline points="16 16 14 18 16 20" />
  <line x1="3" x2="10" y1="18" y2="18" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdWrench;
impl IconShape for LdWrench {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdX;
impl IconShape for LdX {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M18 6 6 18" />
  <path d="m6 6 12 12" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdYoutube;
impl IconShape for LdYoutube {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17" />
  <path d="m10 15 5-3-5-3z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdZapOff;
impl IconShape for LdZapOff {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M10.513 4.856 13.12 2.17a.5.5 0 0 1 .86.46l-1.377 4.317" />
  <path d="M15.656 10H20a1 1 0 0 1 .78 1.63l-1.72 1.773" />
  <path d="M16.273 16.273 10.88 21.83a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14H4a1 1 0 0 1-.78-1.63l4.507-4.643" />
  <path d="m2 2 20 20" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdZap;
impl IconShape for LdZap {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdZoomIn;
impl IconShape for LdZoomIn {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11" cy="11" r="8" />
  <line x1="21" x2="16.65" y1="21" y2="16.65" />
  <line x1="11" x2="11" y1="8" y2="14" />
  <line x1="8" x2="14" y1="11" y2="11" />
</svg>
"#
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LdZoomOut;
impl IconShape for LdZoomOut {
    fn content(&self) -> &'static str {
        r#"<svg
  xmlns="http://www.w3.org/2000/svg"
  
  
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <circle cx="11" cy="11" r="8" />
  <line x1="21" x2="16.65" y1="21" y2="16.65" />
  <line x1="8" x2="14" y1="11" y2="11" />
</svg>
"#
    }
}
