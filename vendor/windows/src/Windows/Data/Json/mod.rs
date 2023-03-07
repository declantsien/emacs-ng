#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArray(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonArray {
    type Vtable = IJsonArray_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonArray {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArray_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetObjectAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetArrayAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStringAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumberAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetBooleanAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArrayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonArrayStatics {
    type Vtable = IJsonArrayStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonArrayStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb1434a9_e164_499f_93e2_8a8f49bb90ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArrayStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonErrorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonErrorStatics2 {
    type Vtable = IJsonErrorStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonErrorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x404030da_87d0_436c_83ab_fc7b12c0cc26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonErrorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetJsonStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonObject {
    type Vtable = IJsonObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObject_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetNamedBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonObjectStatics {
    type Vtable = IJsonObjectStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonObjectStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2289f159_54de_45d8_abcc_22603fa066a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectWithDefaultValues(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonObjectWithDefaultValues {
    type Vtable = IJsonObjectWithDefaultValues_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonObjectWithDefaultValues {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd960d2a2_b7f0_4f00_8e44_d82cf415ea13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectWithDefaultValues_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetNamedValueOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedObjectOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedStringOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedArrayOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNamedNumberOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, defaultvalue: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetNamedBooleanOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, defaultvalue: bool, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct IJsonValue(::windows::core::IUnknown);
impl IJsonValue {
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stringify)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetObject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IJsonValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IJsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IJsonValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsonValue {}
impl ::core::fmt::Debug for IJsonValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsonValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IJsonValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IJsonValue {
    type Vtable = IJsonValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows::core::HRESULT,
    pub Stringify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonValueStatics {
    type Vtable = IJsonValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6b544a_2f53_48e1_91a3_f78b50a6345c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CreateBooleanValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNumberValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJsonValueStatics2 {
    type Vtable = IJsonValueStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IJsonValueStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d9ecbe4_3fe8_4335_8392_93d8e36865f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateNullValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonArray(::windows::core::IUnknown);
impl JsonArray {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonArray, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetObjectAt(&self, index: u32) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetObjectAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetArrayAt(&self, index: u32) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetArrayAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetStringAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStringAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumberAt(&self, index: u32) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumberAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBooleanAt(&self, index: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stringify)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetObject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0, E0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IJsonValue>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0, E0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IJsonValue>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0, E0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IJsonValue>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IJsonValue>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IJsonValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<IJsonValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    #[doc(hidden)]
    pub fn IJsonArrayStatics<R, F: FnOnce(&IJsonArrayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonArray, IJsonArrayStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for JsonArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JsonArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonArray {}
impl ::core::fmt::Debug for JsonArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonArray").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonArray {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonArray;{08c1ddb6-0cbd-4a9a-b5d3-2f852dc37e81})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JsonArray {
    type Vtable = IJsonArray_Vtbl;
}
unsafe impl ::windows::core::Interface for JsonArray {
    const IID: ::windows::core::GUID = <IJsonArray as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JsonArray {
    const NAME: &'static str = "Windows.Data.Json.JsonArray";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for JsonArray {
    type Item = IJsonValue;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &JsonArray {
    type Item = IJsonValue;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(JsonArray, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonArray> for super::super::Foundation::Collections::IIterable<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for super::super::Foundation::Collections::IIterable<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<JsonArray> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonArray> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&JsonArray> for ::windows::core::InParam<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<JsonArray> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonArray> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonArray> for ::windows::core::InParam<super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonArray> for super::super::Foundation::Collections::IVector<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonArray) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for super::super::Foundation::Collections::IVector<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for ::windows::core::InParam<super::super::Foundation::Collections::IVector<IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonArray) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for JsonArray {}
unsafe impl ::core::marker::Sync for JsonArray {}
#[doc = "*Required features: `\"Data_Json\"`*"]
pub struct JsonError;
impl JsonError {
    pub fn GetJsonStatus(hresult: i32) -> ::windows::core::Result<JsonErrorStatus> {
        Self::IJsonErrorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetJsonStatus)(::windows::core::Vtable::as_raw(this), hresult, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJsonErrorStatics2<R, F: FnOnce(&IJsonErrorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonError, IJsonErrorStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for JsonError {
    const NAME: &'static str = "Windows.Data.Json.JsonError";
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonObject(::windows::core::IUnknown);
impl JsonObject {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonObject, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedValue(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNamedValue<P0, E0>(&self, name: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IJsonValue>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNamedValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GetNamedObject(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedObject)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedArray(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedArray)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedString(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedNumber(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedNumber)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedBoolean(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedBoolean)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetNamedValueOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &JsonValue) -> ::windows::core::Result<JsonValue> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedValueOrDefault)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedObjectOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &JsonObject) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedObjectOrDefault)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedStringOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedStringOrDefault)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedArrayOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &JsonArray) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedArrayOrDefault)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(defaultvalue), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedNumberOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: f64) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedNumberOrDefault)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), defaultvalue, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNamedBooleanOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: bool) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNamedBooleanOrDefault)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), defaultvalue, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stringify)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = &::windows::core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetObject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<IJsonValue> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IJsonValue>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0, E0>(&self, key: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IJsonValue>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Insert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), value.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IJsonObjectStatics<R, F: FnOnce(&IJsonObjectStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonObject, IJsonObjectStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for JsonObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JsonObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonObject {}
impl ::core::fmt::Debug for JsonObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonObject;{064e24dd-29c2-4f83-9ac1-9ee11578beb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JsonObject {
    type Vtable = IJsonObject_Vtbl;
}
unsafe impl ::windows::core::Interface for JsonObject {
    const IID: ::windows::core::GUID = <IJsonObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JsonObject {
    const NAME: &'static str = "Windows.Data.Json.JsonObject";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for JsonObject {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &JsonObject {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(JsonObject, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonObject> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IJsonValue>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<JsonObject> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonObject> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&JsonObject> for ::windows::core::InParam<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonObject> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for ::windows::core::InParam<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IJsonValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<JsonObject> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonObject> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonObject> for ::windows::core::InParam<super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonObject) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for JsonObject {}
unsafe impl ::core::marker::Sync for JsonObject {}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonValue(::windows::core::IUnknown);
impl JsonValue {
    pub fn ValueType(&self) -> ::windows::core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stringify)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows::core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetArray)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows::core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetObject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBooleanValue(input: bool) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBooleanValue)(::windows::core::Vtable::as_raw(this), input, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateNumberValue(input: f64) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNumberValue)(::windows::core::Vtable::as_raw(this), input, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateStringValue(input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStringValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateNullValue() -> ::windows::core::Result<JsonValue> {
        Self::IJsonValueStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNullValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IJsonValueStatics<R, F: FnOnce(&IJsonValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonValue, IJsonValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IJsonValueStatics2<R, F: FnOnce(&IJsonValueStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JsonValue, IJsonValueStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for JsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JsonValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonValue {}
impl ::core::fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonValue;{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JsonValue {
    type Vtable = IJsonValue_Vtbl;
}
unsafe impl ::windows::core::Interface for JsonValue {
    const IID: ::windows::core::GUID = <IJsonValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JsonValue {
    const NAME: &'static str = "Windows.Data.Json.JsonValue";
}
::windows::core::interface_hierarchy!(JsonValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<JsonValue> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonValue> for IJsonValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&JsonValue> for ::windows::core::InParam<IJsonValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<JsonValue> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: JsonValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonValue> for super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&JsonValue> for ::windows::core::InParam<super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &JsonValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for JsonValue {}
unsafe impl ::core::marker::Sync for JsonValue {}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsonErrorStatus(pub i32);
impl JsonErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const InvalidJsonString: Self = Self(1i32);
    pub const InvalidJsonNumber: Self = Self(2i32);
    pub const JsonValueNotFound: Self = Self(3i32);
    pub const ImplementationLimit: Self = Self(4i32);
}
impl ::core::marker::Copy for JsonErrorStatus {}
impl ::core::clone::Clone for JsonErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsonErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsonErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsonErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsonValueType(pub i32);
impl JsonValueType {
    pub const Null: Self = Self(0i32);
    pub const Boolean: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const String: Self = Self(3i32);
    pub const Array: Self = Self(4i32);
    pub const Object: Self = Self(5i32);
}
impl ::core::marker::Copy for JsonValueType {}
impl ::core::clone::Clone for JsonValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsonValueType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JsonValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsonValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValueType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JsonValueType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonValueType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");