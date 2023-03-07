#[doc = "*Required features: `\"Graphics_Effects\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffect(::windows::core::IUnknown);
impl IGraphicsEffect {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
}
::windows::core::interface_hierarchy!(IGraphicsEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IGraphicsEffect> for IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: IGraphicsEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IGraphicsEffect> for IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &IGraphicsEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IGraphicsEffect> for ::windows::core::InParam<IGraphicsEffectSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IGraphicsEffect) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IGraphicsEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffect {}
impl ::core::fmt::Debug for IGraphicsEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGraphicsEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cb51c0ce-8fe6-4636-b202-861faa07d8f3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IGraphicsEffect {
    type Vtable = IGraphicsEffect_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb51c0ce_8fe6_4636_b202_861faa07d8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Effects\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffectSource(::windows::core::IUnknown);
impl IGraphicsEffectSource {}
::windows::core::interface_hierarchy!(IGraphicsEffectSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IGraphicsEffectSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffectSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffectSource {}
impl ::core::fmt::Debug for IGraphicsEffectSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffectSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGraphicsEffectSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d8f9ddc-4339-4eb9-9216-f9deb75658a2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IGraphicsEffectSource {
    type Vtable = IGraphicsEffectSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IGraphicsEffectSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d8f9ddc_4339_4eb9_9216_f9deb75658a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");