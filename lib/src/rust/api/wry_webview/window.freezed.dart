// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'window.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$FlutterWindowHandle {

 PlatformInt64 get field0;
/// Create a copy of FlutterWindowHandle
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FlutterWindowHandleCopyWith<FlutterWindowHandle> get copyWith => _$FlutterWindowHandleCopyWithImpl<FlutterWindowHandle>(this as FlutterWindowHandle, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is FlutterWindowHandle&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'FlutterWindowHandle(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $FlutterWindowHandleCopyWith<$Res>  {
  factory $FlutterWindowHandleCopyWith(FlutterWindowHandle value, $Res Function(FlutterWindowHandle) _then) = _$FlutterWindowHandleCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 field0
});




}
/// @nodoc
class _$FlutterWindowHandleCopyWithImpl<$Res>
    implements $FlutterWindowHandleCopyWith<$Res> {
  _$FlutterWindowHandleCopyWithImpl(this._self, this._then);

  final FlutterWindowHandle _self;
  final $Res Function(FlutterWindowHandle) _then;

/// Create a copy of FlutterWindowHandle
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}

}


/// Adds pattern-matching-related methods to [FlutterWindowHandle].
extension FlutterWindowHandlePatterns on FlutterWindowHandle {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( FlutterWindowHandle_Hwnd value)?  hwnd,required TResult orElse(),}){
final _that = this;
switch (_that) {
case FlutterWindowHandle_Hwnd() when hwnd != null:
return hwnd(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( FlutterWindowHandle_Hwnd value)  hwnd,}){
final _that = this;
switch (_that) {
case FlutterWindowHandle_Hwnd():
return hwnd(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( FlutterWindowHandle_Hwnd value)?  hwnd,}){
final _that = this;
switch (_that) {
case FlutterWindowHandle_Hwnd() when hwnd != null:
return hwnd(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( PlatformInt64 field0)?  hwnd,required TResult orElse(),}) {final _that = this;
switch (_that) {
case FlutterWindowHandle_Hwnd() when hwnd != null:
return hwnd(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( PlatformInt64 field0)  hwnd,}) {final _that = this;
switch (_that) {
case FlutterWindowHandle_Hwnd():
return hwnd(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( PlatformInt64 field0)?  hwnd,}) {final _that = this;
switch (_that) {
case FlutterWindowHandle_Hwnd() when hwnd != null:
return hwnd(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class FlutterWindowHandle_Hwnd extends FlutterWindowHandle {
  const FlutterWindowHandle_Hwnd(this.field0): super._();
  

@override final  PlatformInt64 field0;

/// Create a copy of FlutterWindowHandle
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$FlutterWindowHandle_HwndCopyWith<FlutterWindowHandle_Hwnd> get copyWith => _$FlutterWindowHandle_HwndCopyWithImpl<FlutterWindowHandle_Hwnd>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is FlutterWindowHandle_Hwnd&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'FlutterWindowHandle.hwnd(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $FlutterWindowHandle_HwndCopyWith<$Res> implements $FlutterWindowHandleCopyWith<$Res> {
  factory $FlutterWindowHandle_HwndCopyWith(FlutterWindowHandle_Hwnd value, $Res Function(FlutterWindowHandle_Hwnd) _then) = _$FlutterWindowHandle_HwndCopyWithImpl;
@override @useResult
$Res call({
 PlatformInt64 field0
});




}
/// @nodoc
class _$FlutterWindowHandle_HwndCopyWithImpl<$Res>
    implements $FlutterWindowHandle_HwndCopyWith<$Res> {
  _$FlutterWindowHandle_HwndCopyWithImpl(this._self, this._then);

  final FlutterWindowHandle_Hwnd _self;
  final $Res Function(FlutterWindowHandle_Hwnd) _then;

/// Create a copy of FlutterWindowHandle
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(FlutterWindowHandle_Hwnd(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as PlatformInt64,
  ));
}


}

// dart format on
