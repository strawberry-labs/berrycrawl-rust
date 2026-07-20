pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScreenshotDto {
    /// Block common advertising and analytics requests
    #[serde(rename = "blockAds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
    /// Click this CSS selector before capture
    #[serde(rename = "clickSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_selector: Option<String>,
    /// Capture an exact pixel rectangle instead of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<ScreenshotClipDto>,
    #[serde(rename = "colorScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scheme: Option<ScreenshotDtoColorScheme>,
    /// Cookies to set before loading the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<ScreenshotCookieDto>>,
    /// Extra settling time after the page is ready, in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub delay: Option<f64>,
    /// Named viewport preset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<ScreenshotDtoDevice>,
    #[serde(rename = "disableAnimations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_animations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ScreenshotDtoFormat>,
    /// Capture the complete page instead of only the viewport
    #[serde(rename = "fullPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,
    /// auto uses native capture for normal pages and stitched slices for very tall pages
    #[serde(rename = "fullPageAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page_algorithm: Option<ScreenshotDtoFullPageAlgorithm>,
    /// Headers sent while loading the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, serde_json::Value>>,
    /// Show fixed/sticky UI once instead of repeating it in stitched captures
    #[serde(rename = "hideFixedElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_fixed_elements: Option<bool>,
    /// Hide matching elements before capture
    #[serde(rename = "hideSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_selectors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ScreenshotLocationDto>,
    #[serde(rename = "maskColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_color: Option<String>,
    /// Cover matching elements with a solid privacy mask
    #[serde(rename = "maskSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_selectors: Option<Vec<String>>,
    /// Maximum full-page height. Prevents endless captures on infinite pages.
    #[serde(rename = "maxHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_height: Option<f64>,
    /// Use a transparent background where the page allows it
    #[serde(rename = "omitBackground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_background: Option<bool>,
    /// Proxy mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ScreenshotDtoProxy>,
    /// JPEG or WebP quality
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quality: Option<f64>,
    #[serde(rename = "reducedMotion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduced_motion: Option<bool>,
    /// Hide common support and chat widgets
    #[serde(rename = "removeChatWidgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_chat_widgets: Option<bool>,
    /// Accept known consent dialogs, hide remaining cookie banners, and restore page scrolling
    #[serde(rename = "removeCookieBanners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_cookie_banners: Option<bool>,
    /// Remove newsletter gates, modal backdrops, and blocking overlays
    #[serde(rename = "removeOverlays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_overlays: Option<bool>,
    /// Return a CDN URL or an inline data URL
    #[serde(rename = "responseFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ScreenshotDtoResponseFormat>,
    /// Capture at CSS pixel size or the emulated device pixel ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScreenshotDtoScale>,
    /// Pause between lazy-load scroll steps, in milliseconds
    #[serde(rename = "scrollDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub scroll_delay: Option<f64>,
    /// Scroll through the page first so lazy content is loaded
    #[serde(rename = "scrollPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_page: Option<bool>,
    /// Capture one element instead of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// CSS rules to apply before capture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout: Option<f64>,
    /// Public webpage URL to capture
    #[serde(default)]
    pub url: String,
    /// Custom viewport. Overrides the named device dimensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<ScreenshotViewportDto>,
    /// Wait for this CSS selector before capture
    #[serde(rename = "waitForSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_selector: Option<String>,
    /// Page readiness milestone used before capture
    #[serde(rename = "waitUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<ScreenshotDtoWaitUntil>,
}

impl ScreenshotDto {
    pub fn builder() -> ScreenshotDtoBuilder {
        <ScreenshotDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScreenshotDtoBuilder {
    block_ads: Option<bool>,
    click_selector: Option<String>,
    clip: Option<ScreenshotClipDto>,
    color_scheme: Option<ScreenshotDtoColorScheme>,
    cookies: Option<Vec<ScreenshotCookieDto>>,
    delay: Option<f64>,
    device: Option<ScreenshotDtoDevice>,
    disable_animations: Option<bool>,
    format: Option<ScreenshotDtoFormat>,
    full_page: Option<bool>,
    full_page_algorithm: Option<ScreenshotDtoFullPageAlgorithm>,
    headers: Option<HashMap<String, serde_json::Value>>,
    hide_fixed_elements: Option<bool>,
    hide_selectors: Option<Vec<String>>,
    location: Option<ScreenshotLocationDto>,
    mask_color: Option<String>,
    mask_selectors: Option<Vec<String>>,
    max_height: Option<f64>,
    omit_background: Option<bool>,
    proxy: Option<ScreenshotDtoProxy>,
    quality: Option<f64>,
    reduced_motion: Option<bool>,
    remove_chat_widgets: Option<bool>,
    remove_cookie_banners: Option<bool>,
    remove_overlays: Option<bool>,
    response_format: Option<ScreenshotDtoResponseFormat>,
    scale: Option<ScreenshotDtoScale>,
    scroll_delay: Option<f64>,
    scroll_page: Option<bool>,
    selector: Option<String>,
    styles: Option<Vec<String>>,
    timeout: Option<f64>,
    url: Option<String>,
    viewport: Option<ScreenshotViewportDto>,
    wait_for_selector: Option<String>,
    wait_until: Option<ScreenshotDtoWaitUntil>,
}

impl ScreenshotDtoBuilder {
    pub fn block_ads(mut self, value: bool) -> Self {
        self.block_ads = Some(value);
        self
    }

    pub fn click_selector(mut self, value: impl Into<String>) -> Self {
        self.click_selector = Some(value.into());
        self
    }

    pub fn clip(mut self, value: ScreenshotClipDto) -> Self {
        self.clip = Some(value);
        self
    }

    pub fn color_scheme(mut self, value: ScreenshotDtoColorScheme) -> Self {
        self.color_scheme = Some(value);
        self
    }

    pub fn cookies(mut self, value: Vec<ScreenshotCookieDto>) -> Self {
        self.cookies = Some(value);
        self
    }

    pub fn delay(mut self, value: f64) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn device(mut self, value: ScreenshotDtoDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn disable_animations(mut self, value: bool) -> Self {
        self.disable_animations = Some(value);
        self
    }

    pub fn format(mut self, value: ScreenshotDtoFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn full_page(mut self, value: bool) -> Self {
        self.full_page = Some(value);
        self
    }

    pub fn full_page_algorithm(mut self, value: ScreenshotDtoFullPageAlgorithm) -> Self {
        self.full_page_algorithm = Some(value);
        self
    }

    pub fn headers(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn hide_fixed_elements(mut self, value: bool) -> Self {
        self.hide_fixed_elements = Some(value);
        self
    }

    pub fn hide_selectors(mut self, value: Vec<String>) -> Self {
        self.hide_selectors = Some(value);
        self
    }

    pub fn location(mut self, value: ScreenshotLocationDto) -> Self {
        self.location = Some(value);
        self
    }

    pub fn mask_color(mut self, value: impl Into<String>) -> Self {
        self.mask_color = Some(value.into());
        self
    }

    pub fn mask_selectors(mut self, value: Vec<String>) -> Self {
        self.mask_selectors = Some(value);
        self
    }

    pub fn max_height(mut self, value: f64) -> Self {
        self.max_height = Some(value);
        self
    }

    pub fn omit_background(mut self, value: bool) -> Self {
        self.omit_background = Some(value);
        self
    }

    pub fn proxy(mut self, value: ScreenshotDtoProxy) -> Self {
        self.proxy = Some(value);
        self
    }

    pub fn quality(mut self, value: f64) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn reduced_motion(mut self, value: bool) -> Self {
        self.reduced_motion = Some(value);
        self
    }

    pub fn remove_chat_widgets(mut self, value: bool) -> Self {
        self.remove_chat_widgets = Some(value);
        self
    }

    pub fn remove_cookie_banners(mut self, value: bool) -> Self {
        self.remove_cookie_banners = Some(value);
        self
    }

    pub fn remove_overlays(mut self, value: bool) -> Self {
        self.remove_overlays = Some(value);
        self
    }

    pub fn response_format(mut self, value: ScreenshotDtoResponseFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn scale(mut self, value: ScreenshotDtoScale) -> Self {
        self.scale = Some(value);
        self
    }

    pub fn scroll_delay(mut self, value: f64) -> Self {
        self.scroll_delay = Some(value);
        self
    }

    pub fn scroll_page(mut self, value: bool) -> Self {
        self.scroll_page = Some(value);
        self
    }

    pub fn selector(mut self, value: impl Into<String>) -> Self {
        self.selector = Some(value.into());
        self
    }

    pub fn styles(mut self, value: Vec<String>) -> Self {
        self.styles = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn viewport(mut self, value: ScreenshotViewportDto) -> Self {
        self.viewport = Some(value);
        self
    }

    pub fn wait_for_selector(mut self, value: impl Into<String>) -> Self {
        self.wait_for_selector = Some(value.into());
        self
    }

    pub fn wait_until(mut self, value: ScreenshotDtoWaitUntil) -> Self {
        self.wait_until = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScreenshotDto`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ScreenshotDtoBuilder::url)
    pub fn build(self) -> Result<ScreenshotDto, BuildError> {
        Ok(ScreenshotDto {
            block_ads: self.block_ads,
            click_selector: self.click_selector,
            clip: self.clip,
            color_scheme: self.color_scheme,
            cookies: self.cookies,
            delay: self.delay,
            device: self.device,
            disable_animations: self.disable_animations,
            format: self.format,
            full_page: self.full_page,
            full_page_algorithm: self.full_page_algorithm,
            headers: self.headers,
            hide_fixed_elements: self.hide_fixed_elements,
            hide_selectors: self.hide_selectors,
            location: self.location,
            mask_color: self.mask_color,
            mask_selectors: self.mask_selectors,
            max_height: self.max_height,
            omit_background: self.omit_background,
            proxy: self.proxy,
            quality: self.quality,
            reduced_motion: self.reduced_motion,
            remove_chat_widgets: self.remove_chat_widgets,
            remove_cookie_banners: self.remove_cookie_banners,
            remove_overlays: self.remove_overlays,
            response_format: self.response_format,
            scale: self.scale,
            scroll_delay: self.scroll_delay,
            scroll_page: self.scroll_page,
            selector: self.selector,
            styles: self.styles,
            timeout: self.timeout,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            viewport: self.viewport,
            wait_for_selector: self.wait_for_selector,
            wait_until: self.wait_until,
        })
    }
}
