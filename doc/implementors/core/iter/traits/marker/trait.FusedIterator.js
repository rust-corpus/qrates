(function() {var implementors = {};
implementors["bstr"] = [{"text":"impl&lt;'a&gt; FusedIterator for Bytes&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for DrainBytes&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for CharIndices&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Utf8Chunks&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["cargo"] = [{"text":"impl&lt;'a&gt; FusedIterator for ManifestCauses&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;T&gt; FusedIterator for RawIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; FusedIterator for RawIntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; FusedIterator for RawDrain&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V, F&gt; FusedIterator for DrainFilter&lt;'_, K, V, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;K, &amp;mut V) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Iter&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for IterMut&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for IntoIter&lt;K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Keys&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Values&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for ValuesMut&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, V&gt; FusedIterator for Drain&lt;'_, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; FusedIterator for Iter&lt;'_, K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; FusedIterator for IntoIter&lt;K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K&gt; FusedIterator for Drain&lt;'_, K&gt;","synthetic":false,"types":[]},{"text":"impl&lt;K, F&gt; FusedIterator for DrainFilter&lt;'_, K, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnMut(&amp;K) -&gt; bool,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for Intersection&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for Difference&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for SymmetricDifference&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FusedIterator for Union&lt;'_, T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl&lt;'a, T&gt; FusedIterator for Iter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for IterMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for Keys&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for Values&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for ValuesMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for Drain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for ValueIter&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for ValueIterMut&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; FusedIterator for IntoIter&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; FusedIterator for ValueDrain&lt;'a, T&gt;","synthetic":false,"types":[]}];
implementors["im_rc"] = [{"text":"impl&lt;'a, K, V&gt; FusedIterator for Iter&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; FusedIterator for IterMut&lt;'a, K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A&gt; FusedIterator for ConsumingIter&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: HashValue + Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; FusedIterator for Keys&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V&gt; FusedIterator for Values&lt;'a, K, V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A&gt; FusedIterator for Iter&lt;'a, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A&gt; FusedIterator for IterMut&lt;'a, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A&gt; FusedIterator for ConsumingIter&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Hash + Eq + Clone,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A:&nbsp;Clone&gt; FusedIterator for Iter&lt;'a, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A:&nbsp;Clone&gt; FusedIterator for IterMut&lt;'a, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Clone&gt; FusedIterator for ConsumingIter&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A:&nbsp;Clone&gt; FusedIterator for Chunks&lt;'a, A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A:&nbsp;Clone&gt; FusedIterator for ChunksMut&lt;'a, A&gt;","synthetic":false,"types":[]}];
implementors["ipnet"] = [{"text":"impl FusedIterator for IpAddrRange","synthetic":false,"types":[]},{"text":"impl FusedIterator for Ipv4AddrRange","synthetic":false,"types":[]},{"text":"impl FusedIterator for Ipv6AddrRange","synthetic":false,"types":[]},{"text":"impl FusedIterator for IpSubnets","synthetic":false,"types":[]},{"text":"impl FusedIterator for Ipv4Subnets","synthetic":false,"types":[]},{"text":"impl FusedIterator for Ipv6Subnets","synthetic":false,"types":[]}];
implementors["mime_guess"] = [{"text":"impl FusedIterator for Iter","synthetic":false,"types":[]},{"text":"impl FusedIterator for IterRaw","synthetic":false,"types":[]}];
implementors["rand"] = [{"text":"impl&lt;D, R, T&gt; FusedIterator for DistIter&lt;D, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Distribution&lt;T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rng,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl&lt;'de, R, T&gt; FusedIterator for StreamDeserializer&lt;'de, R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read&lt;'de&gt; + Fused,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Deserialize&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Iter&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for IterMut&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl FusedIterator for IntoIter","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Keys&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for Values&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; FusedIterator for ValuesMut&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["sized_chunks"] = [{"text":"impl&lt;A, T&gt; FusedIterator for Iter&lt;A, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, T&gt; FusedIterator for Drain&lt;'a, A, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, N&gt; FusedIterator for Iter&lt;'a, A, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ChunkLength&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, N&gt; FusedIterator for IterMut&lt;'a, A, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ChunkLength&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A:&nbsp;'a, N:&nbsp;ChunkLength&lt;A&gt; + 'a&gt; FusedIterator for Drain&lt;'a, A, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A, N:&nbsp;ChunkLength&lt;A&gt;&gt; FusedIterator for OwnedIter&lt;A, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A, N&gt; FusedIterator for Iter&lt;A, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ChunkLength&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, N&gt; FusedIterator for Drain&lt;'a, A, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ChunkLength&lt;A&gt; + 'a,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;'p, A, I&gt; FusedIterator for ArrayVecSplice&lt;'p, A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = A::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; FusedIterator for ArrayVecIterator&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;'a + Default&gt; FusedIterator for ArrayVecDrain&lt;'a, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, 's, T:&nbsp;Default&gt; FusedIterator for SliceVecDrain&lt;'p, 's, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p, A, I&gt; FusedIterator for TinyVecSplice&lt;'p, A, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Array,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator&lt;Item = A::Item&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; FusedIterator for TinyVecIterator&lt;A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()