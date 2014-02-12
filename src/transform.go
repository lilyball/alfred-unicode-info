package main

// #cgo LDFLAGS: -framework CoreFoundation
// #include <CoreFoundation/CoreFoundation.h>
import "C"

import (
	"errors"
	"unsafe"
)

func transform(input, transform string, reverse bool) (string, error) {
	incstr := C.CString(input)
	defer C.free(unsafe.Pointer(incstr))
	tcstr := C.CString(transform)
	defer C.free(unsafe.Pointer(tcstr))
	cfstr := C.CFStringCreateMutable(nil, C.CFIndex(0))
	if cfstr == nil {
		return "", errors.New("Could not create CFMutableStringRef for input")
	}
	defer C.CFRelease(C.CFTypeRef(cfstr))

	C.CFStringAppendCString(cfstr, incstr, C.kCFStringEncodingUTF8)

	tstr := C.CFStringCreateWithCString(nil, tcstr, C.kCFStringEncodingUTF8)
	if tstr == nil {
		return "", errors.New("Could not create CStringRef for transform")
	}
	defer C.CFRelease(C.CFTypeRef(tstr))

	if booleanToBool(C.CFStringTransform(cfstr, nil, tstr, boolToBoolean(reverse))) {
		cfdata := C.CFStringCreateExternalRepresentation(nil, cfstr, C.kCFStringEncodingUTF8, C.UInt8(0))
		if cfdata == nil {
			return "", errors.New("Could not create external representation")
		}
		defer C.CFRelease(C.CFTypeRef(cfdata))
		output := C.GoStringN((*C.char)(unsafe.Pointer(C.CFDataGetBytePtr(cfdata))), C.int(C.CFDataGetLength(cfdata)))
		return output, nil
	}
	return "", errors.New("Could not transform input")
}

func boolToBoolean(b bool) C.Boolean {
	if b {
		return C.true
	}
	return C.false
}

func booleanToBool(b C.Boolean) bool {
	if b == C.false {
		return false
	}
	return true
}
