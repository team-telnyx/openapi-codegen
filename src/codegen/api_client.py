class ApiClient:
    """
    The HTTP API client
    """

    def __init__(
        self, base_url: str, session: Optional[aiohttp.ClientSession] = None
    ) -> None:
        self._base_url = base_url

        if session:
            self._session = session
        else:
            self._session = aiohttp.ClientSession()
