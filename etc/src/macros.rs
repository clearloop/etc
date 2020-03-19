//! etc macros

/// Macro for easily creating a new implementation of the `Get` trait. Use similarly to
/// how you would declare a `const`:
///
/// ```no_compile
/// parameter_types! {
///   pub const Argument: u64 = 42;
/// }
/// trait Config {
///   type Parameter: Get<u64>;
/// }
/// struct Runtime;
/// impl Config for Runtime {
///   type Parameter = Argument;
/// }
/// ```
#[macro_export]
macro_rules! parameter_types {
	(
		$( #[ $attr:meta ] )*
		$vis:vis const $name:ident: $type:ty = $value:expr;
		$( $rest:tt )*
	) => (
		$( #[ $attr ] )*
		$vis struct $name;
		$crate::parameter_types!{IMPL $name , $type , $value}
		$crate::parameter_types!{ $( $rest )* }
	);
	() => ();
	(IMPL $name:ident , $type:ty , $value:expr) => {
		impl $name {
			pub fn get() -> $type {
				$value
			}
		}
		impl<I: From<$type>> $crate::traits::Get<I> for $name {
			fn get() -> I {
				I::from($value)
			}
		}
	}
}
