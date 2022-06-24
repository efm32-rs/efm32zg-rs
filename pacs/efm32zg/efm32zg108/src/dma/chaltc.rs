#[doc = "Register `CHALTC` writer"]
pub struct W(crate::W<CHALTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHALTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHALTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHALTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ALTC` writer - Channel 0 Alternate Clear"]
pub type CH0ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 0>;
#[doc = "Field `CH1ALTC` writer - Channel 1 Alternate Clear"]
pub type CH1ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 1>;
#[doc = "Field `CH2ALTC` writer - Channel 2 Alternate Clear"]
pub type CH2ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 2>;
#[doc = "Field `CH3ALTC` writer - Channel 3 Alternate Clear"]
pub type CH3ALTC_W<'a> = crate::BitWriter<'a, u32, CHALTC_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Clear"]
    #[inline(always)]
    pub fn ch0altc(&mut self) -> CH0ALTC_W {
        CH0ALTC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Clear"]
    #[inline(always)]
    pub fn ch1altc(&mut self) -> CH1ALTC_W {
        CH1ALTC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Clear"]
    #[inline(always)]
    pub fn ch2altc(&mut self) -> CH2ALTC_W {
        CH2ALTC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Clear"]
    #[inline(always)]
    pub fn ch3altc(&mut self) -> CH3ALTC_W {
        CH3ALTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Alternate Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chaltc](index.html) module"]
pub struct CHALTC_SPEC;
impl crate::RegisterSpec for CHALTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chaltc::W](W) writer structure"]
impl crate::Writable for CHALTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHALTC to value 0"]
impl crate::Resettable for CHALTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
