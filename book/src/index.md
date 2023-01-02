<script>
	const userLocale = navigator.languages && navigator.languages.length ? navigator.languages[0] : navigator.language;

	if (userLocale.includes("es")) {
		window.location.href = window.location.origin + "/cargo-is-tested/es/chapter_1.html"
	} else {
		window.location.href = window.location.origin + "/cargo-is-tested/en/chapter_1.html"
	}
</script>
