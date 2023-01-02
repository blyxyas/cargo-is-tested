<script>
	const userLocale = navigator.languages && navigator.languages.length ? navigator.languages[0] : navigator.language;

	if (userLocale.includes("es")) {
		window.location.href = window.location.origin + "/es/chapter_1.html"
	} else {
		window.location.href = window.location.origin + "/en/chapter_1.html"
	}
</script>
