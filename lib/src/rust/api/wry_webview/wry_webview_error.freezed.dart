// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'wry_webview_error.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$WryWebViewError {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WryWebViewError);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'WryWebViewError()';
}


}

/// @nodoc
class $WryWebViewErrorCopyWith<$Res>  {
$WryWebViewErrorCopyWith(WryWebViewError _, $Res Function(WryWebViewError) __);
}


/// Adds pattern-matching-related methods to [WryWebViewError].
extension WryWebViewErrorPatterns on WryWebViewError {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( WryWebViewError_CreationError value)?  creationError,TResult Function( WryWebViewError_UnsupportedPlatform value)?  unsupportedPlatform,required TResult orElse(),}){
final _that = this;
switch (_that) {
case WryWebViewError_CreationError() when creationError != null:
return creationError(_that);case WryWebViewError_UnsupportedPlatform() when unsupportedPlatform != null:
return unsupportedPlatform(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( WryWebViewError_CreationError value)  creationError,required TResult Function( WryWebViewError_UnsupportedPlatform value)  unsupportedPlatform,}){
final _that = this;
switch (_that) {
case WryWebViewError_CreationError():
return creationError(_that);case WryWebViewError_UnsupportedPlatform():
return unsupportedPlatform(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( WryWebViewError_CreationError value)?  creationError,TResult? Function( WryWebViewError_UnsupportedPlatform value)?  unsupportedPlatform,}){
final _that = this;
switch (_that) {
case WryWebViewError_CreationError() when creationError != null:
return creationError(_that);case WryWebViewError_UnsupportedPlatform() when unsupportedPlatform != null:
return unsupportedPlatform(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String field0)?  creationError,TResult Function()?  unsupportedPlatform,required TResult orElse(),}) {final _that = this;
switch (_that) {
case WryWebViewError_CreationError() when creationError != null:
return creationError(_that.field0);case WryWebViewError_UnsupportedPlatform() when unsupportedPlatform != null:
return unsupportedPlatform();case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String field0)  creationError,required TResult Function()  unsupportedPlatform,}) {final _that = this;
switch (_that) {
case WryWebViewError_CreationError():
return creationError(_that.field0);case WryWebViewError_UnsupportedPlatform():
return unsupportedPlatform();}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String field0)?  creationError,TResult? Function()?  unsupportedPlatform,}) {final _that = this;
switch (_that) {
case WryWebViewError_CreationError() when creationError != null:
return creationError(_that.field0);case WryWebViewError_UnsupportedPlatform() when unsupportedPlatform != null:
return unsupportedPlatform();case _:
  return null;

}
}

}

/// @nodoc


class WryWebViewError_CreationError extends WryWebViewError {
  const WryWebViewError_CreationError(this.field0): super._();
  

 final  String field0;

/// Create a copy of WryWebViewError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$WryWebViewError_CreationErrorCopyWith<WryWebViewError_CreationError> get copyWith => _$WryWebViewError_CreationErrorCopyWithImpl<WryWebViewError_CreationError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WryWebViewError_CreationError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'WryWebViewError.creationError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $WryWebViewError_CreationErrorCopyWith<$Res> implements $WryWebViewErrorCopyWith<$Res> {
  factory $WryWebViewError_CreationErrorCopyWith(WryWebViewError_CreationError value, $Res Function(WryWebViewError_CreationError) _then) = _$WryWebViewError_CreationErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$WryWebViewError_CreationErrorCopyWithImpl<$Res>
    implements $WryWebViewError_CreationErrorCopyWith<$Res> {
  _$WryWebViewError_CreationErrorCopyWithImpl(this._self, this._then);

  final WryWebViewError_CreationError _self;
  final $Res Function(WryWebViewError_CreationError) _then;

/// Create a copy of WryWebViewError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(WryWebViewError_CreationError(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class WryWebViewError_UnsupportedPlatform extends WryWebViewError {
  const WryWebViewError_UnsupportedPlatform(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is WryWebViewError_UnsupportedPlatform);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'WryWebViewError.unsupportedPlatform()';
}


}




// dart format on
