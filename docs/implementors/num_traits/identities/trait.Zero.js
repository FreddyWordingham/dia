(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, C:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"type\" href=\"nalgebra/base/type.MatrixMN.html\" title=\"type nalgebra::base::MatrixMN\">MatrixMN</a>&lt;N, R, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedAdd.html\" title=\"trait nalgebra::ClosedAdd\">ClosedAdd</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::base::alias::MatrixMN"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Quaternion.html\" title=\"struct nalgebra::geometry::Quaternion\">Quaternion</a>&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N::<a class=\"type\" href=\"simba/simd/simd_value/trait.SimdValue.html#associatedtype.Element\" title=\"type simba::simd::simd_value::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::quaternion::Quaternion"]}];
implementors["ndarray"] = [{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 0]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]},{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 1]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]},{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 2]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]},{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 3]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]},{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 4]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]},{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 5]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]},{"text":"impl <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"ndarray/struct.Dim.html\" title=\"struct ndarray::Dim\">Dim</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"type\" href=\"ndarray/type.Ix.html\" title=\"type ndarray::Ix\">Ix</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; 6]</a>&gt;","synthetic":false,"types":["ndarray::dimension::dim::Dim"]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;","synthetic":false,"types":["num_complex::Complex"]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"num_integer/trait.Integer.html\" title=\"trait num_integer::Integer\">Integer</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> for <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;","synthetic":false,"types":["num_rational::Ratio"]}];
implementors["num_traits"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()