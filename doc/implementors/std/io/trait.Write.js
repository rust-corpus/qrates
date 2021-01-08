(function() {var implementors = {};
implementors["base64"] = [{"text":"impl&lt;'a, W:&nbsp;Write&gt; Write for EncoderWriter&lt;'a, W&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl&lt;B:&nbsp;BufMut + Sized&gt; Write for Writer&lt;B&gt;","synthetic":false,"types":[]}];
implementors["cargo"] = [{"text":"impl Write for FileLock","synthetic":false,"types":[]}];
implementors["crypto_hash"] = [{"text":"impl Write for Hasher","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Write for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Write,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Write,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["env_logger"] = [{"text":"impl Write for Formatter","synthetic":false,"types":[]}];
implementors["flate2"] = [{"text":"impl&lt;W:&nbsp;Write&gt; Write for CrcWriter&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;BufRead + Write&gt; Write for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;BufRead + Write&gt; Write for DeflateDecoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Write for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Write for DeflateDecoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for DeflateDecoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead + Write&gt; Write for GzEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead + Write&gt; Write for GzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead + Write&gt; Write for MultiGzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read + Write&gt; Write for GzEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read + Write&gt; Write for GzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read + Write&gt; Write for MultiGzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for GzEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for GzDecoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead + Write&gt; Write for ZlibEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead + Write&gt; Write for ZlibDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Write for ZlibEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read + Write&gt; Write for ZlibDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for ZlibEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for ZlibDecoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;T&gt; Write for AllowStdIo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Write,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["git2"] = [{"text":"impl&lt;'repo&gt; Write for BlobWriter&lt;'repo&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'repo&gt; Write for OdbWriter&lt;'repo&gt;","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl Write for AddrStream","synthetic":false,"types":[]},{"text":"impl Write for Upgraded","synthetic":false,"types":[]}];
implementors["hyper_tls"] = [{"text":"impl&lt;T:&nbsp;Read + Write&gt; Write for MaybeHttpsStream&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Read + Write&gt; Write for TlsStream&lt;T&gt;","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl Write for TcpStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a TcpStream","synthetic":false,"types":[]}];
implementors["mio_uds"] = [{"text":"impl Write for UnixStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a UnixStream","synthetic":false,"types":[]}];
implementors["native_tls"] = [{"text":"impl&lt;S:&nbsp;Read + Write&gt; Write for TlsStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl Write for Hasher","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for Signer&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for Verifier&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;Read + Write&gt; Write for SslStream&lt;S&gt;","synthetic":false,"types":[]}];
implementors["sized_chunks"] = [{"text":"impl&lt;N:&nbsp;ChunkLength&lt;u8&gt;&gt; Write for RingBuffer&lt;u8, N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N&gt; Write for Chunk&lt;u8, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ChunkLength&lt;u8&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&lt;Item = u8&gt;&gt; Write for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["socket2"] = [{"text":"impl Write for Socket","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a Socket","synthetic":false,"types":[]}];
implementors["strip_ansi_escapes"] = [{"text":"impl&lt;W&gt; Write for Writer&lt;W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Write,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tempfile"] = [{"text":"impl Write for NamedTempFile","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a NamedTempFile","synthetic":false,"types":[]},{"text":"impl Write for SpooledTempFile","synthetic":false,"types":[]}];
implementors["termcolor"] = [{"text":"impl Write for StandardStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for StandardStreamLock&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Write for BufferedStandardStream","synthetic":false,"types":[]},{"text":"impl Write for Buffer","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for NoColor&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Write&gt; Write for Ansi&lt;W&gt;","synthetic":false,"types":[]}];
implementors["tokio_fs"] = [{"text":"impl Write for File","synthetic":false,"types":[]},{"text":"impl Write for Stderr","synthetic":false,"types":[]},{"text":"impl Write for Stdout","synthetic":false,"types":[]}];
implementors["tokio_io"] = [{"text":"impl&lt;T&gt; Write for AllowStdIo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Write,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;AsyncWrite&gt; Write for WriteHalf&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_process"] = [{"text":"impl Write for ChildStdin","synthetic":false,"types":[]}];
implementors["tokio_reactor"] = [{"text":"impl&lt;E&gt; Write for PollEvented&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Evented + Write,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, E&gt; Write for &amp;'a PollEvented&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Evented,<br>&nbsp;&nbsp;&nbsp;&nbsp;&amp;'a E: Write,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tokio_tcp"] = [{"text":"impl Write for TcpStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a TcpStream","synthetic":false,"types":[]}];
implementors["tokio_uds"] = [{"text":"impl Write for UnixStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Write for &amp;'a UnixStream","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()