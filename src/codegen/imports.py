from typing import Optional, List, Set, Dict, Any

import aiohttp
from pydantic import BaseModel, Field

class _BaseModel(BaseModel):
    class Config:
        allow_population_by_field_name = True
