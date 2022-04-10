(function() {var implementors = {};
implementors["ordered_varint"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.i8.html\">i8</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.i16.html\">i16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.i32.html\">i32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.i64.html\">i64</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.isize.html\">isize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.i128.html\">i128</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.isize.html\">isize</a>&gt; for <a class=\"struct\" href=\"ordered_varint/struct.Signed.html\" title=\"struct ordered_varint::Signed\">Signed</a>","synthetic":false,"types":["ordered_varint::signed::Signed"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u32.html\">u32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u64.html\">u64</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u128.html\">u128</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"ordered_varint/struct.Unsigned.html\" title=\"struct ordered_varint::Unsigned\">Unsigned</a>","synthetic":false,"types":["ordered_varint::unsigned::Unsigned"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()