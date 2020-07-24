package main

import "testing"

//balanced("xxxyyy") => true
//balanced("yyyxxx") => true
//balanced("xxxyyyy") => false
//balanced("yyxyxxyxxyyyyxxxyxyx") => true
//balanced("xyxxxxyyyxyxxyxxyy") => false
//balanced("") => true
//balanced("x") => false

func TestHello(t *testing.T) {
	t.Run("balanced(\"xxxyyy\") => true", func(t *testing.T) {
		isBalanced := balanced("xxxyyy")
		if !isBalanced {
			t.Errorf("balanced(\"xxxyyy\") => true returned false")
		}
	})
	t.Run("balanced(\"yyyxxx\") => true", func(t *testing.T) {
		isBalanced := balanced("yyyxxx")
		if !isBalanced {
			t.Errorf("balanced(\"yyyxxx\") => true returned false")
		}
	})
	t.Run("balanced(\"yyxyxxyxxyyyyxxxyxyx\") => true", func(t *testing.T) {
		isBalanced := balanced("yyxyxxyxxyyyyxxxyxyx")
		if !isBalanced {
			t.Errorf("balanced(\"yyxyxxyxxyyyyxxxyxyx\") => true returned false")
		}
	})
	t.Run("balanced(\"\") => true", func(t *testing.T) {
		isBalanced := balanced("")
		if !isBalanced {
			t.Errorf("balanced(\"\") => true returned false")
		}
	})
	t.Run("balanced(\"xxxyyyy\") => false", func(t *testing.T) {
		isBalanced := balanced("xxxyyyy")
		if isBalanced {
			t.Errorf("balanced(\"xxxyyyy\") => false returned true")
		}
	})
	t.Run("balanced(\"xyxxxxyyyxyxxyxxyy\") => false", func(t *testing.T) {
		isBalanced := balanced("xyxxxxyyyxyxxyxxyy")
		if isBalanced {
			t.Errorf("balanced(\"xyxxxxyyyxyxxyxxyy\") => false returned true")
		}
	})
	t.Run("balanced(\"x\") => false", func(t *testing.T) {
		isBalanced := balanced("x")
		if isBalanced {
			t.Errorf("balanced(\"x\") => false returned true")
		}
	})
}
