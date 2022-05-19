from typing import Optional, List, Set, Dict, Any, Union, Tuple

import aiohttp
from pydantic import BaseModel, Field, parse_obj_as

class _BaseModel(BaseModel):
    class Config:
        allow_population_by_field_name = True
