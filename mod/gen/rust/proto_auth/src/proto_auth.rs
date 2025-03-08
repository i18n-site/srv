// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum SignupMailVerify {
  OK = 0,
  INVALID_ADDRESS = 1,
  PASSWORD_TOO_SHORT = 2,
}
impl SignupMailVerify {
  pub const KNOWN_VARIANTS: [SignupMailVerify; 3] = [SignupMailVerify::OK, SignupMailVerify::INVALID_ADDRESS, SignupMailVerify::PASSWORD_TOO_SHORT];
}
impl ::std::default::Default for SignupMailVerify {
  fn default() -> Self {
    SignupMailVerify::OK
  }
}
impl From<SignupMailVerify> for i32 {
  fn from(v: SignupMailVerify) -> i32 {
    match v {
      SignupMailVerify::OK => 0,
      SignupMailVerify::INVALID_ADDRESS => 1,
      SignupMailVerify::PASSWORD_TOO_SHORT => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for SignupMailVerify {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(SignupMailVerify::OK),
      1 => Ok(SignupMailVerify::INVALID_ADDRESS),
      2 => Ok(SignupMailVerify::PASSWORD_TOO_SHORT),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for SignupMailVerify {
}
impl ::pb_jelly::ClosedProtoEnum for SignupMailVerify {
  fn name(self) -> &'static str {
    match self {
      SignupMailVerify::OK => "OK",
      SignupMailVerify::INVALID_ADDRESS => "INVALID_ADDRESS",
      SignupMailVerify::PASSWORD_TOO_SHORT => "PASSWORD_TOO_SHORT",
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SigninMailArgs {
  pub address: ::std::string::String,
  pub password: ::std::string::String,
}
impl ::std::default::Default for SigninMailArgs {
  fn default() -> Self {
    SigninMailArgs {
      address: ::std::default::Default::default(),
      password: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref SigninMailArgs_default: SigninMailArgs = SigninMailArgs::default();
}
impl ::pb_jelly::Message for SigninMailArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SigninMailArgs",
      full_name: "auth.SigninMailArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "address",
          full_name: "auth.SigninMailArgs.address",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "password",
          full_name: "auth.SigninMailArgs.password",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SigninMailArgs", 1)?;
          self.address = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SigninMailArgs", 2)?;
          self.password = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SigninMailArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "address" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.address)
      }
      "password" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.password)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignupMailArgs {
  pub address: ::std::string::String,
  pub password: ::std::string::String,
}
impl ::std::default::Default for SignupMailArgs {
  fn default() -> Self {
    SignupMailArgs {
      address: ::std::default::Default::default(),
      password: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref SignupMailArgs_default: SignupMailArgs = SignupMailArgs::default();
}
impl ::pb_jelly::Message for SignupMailArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SignupMailArgs",
      full_name: "auth.SignupMailArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "address",
          full_name: "auth.SignupMailArgs.address",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "password",
          full_name: "auth.SignupMailArgs.password",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SignupMailArgs", 1)?;
          self.address = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SignupMailArgs", 2)?;
          self.password = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SignupMailArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "address" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.address)
      }
      "password" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.password)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignupMailVerifyArgs {
  pub address: ::std::string::String,
  pub code: ::std::string::String,
}
impl ::std::default::Default for SignupMailVerifyArgs {
  fn default() -> Self {
    SignupMailVerifyArgs {
      address: ::std::default::Default::default(),
      code: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref SignupMailVerifyArgs_default: SignupMailVerifyArgs = SignupMailVerifyArgs::default();
}
impl ::pb_jelly::Message for SignupMailVerifyArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SignupMailVerifyArgs",
      full_name: "auth.SignupMailVerifyArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "address",
          full_name: "auth.SignupMailVerifyArgs.address",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "code",
          full_name: "auth.SignupMailVerifyArgs.code",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.code, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.code, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SignupMailVerifyArgs", 1)?;
          self.address = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SignupMailVerifyArgs", 2)?;
          self.code = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SignupMailVerifyArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "address" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.address)
      }
      "code" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.code)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

