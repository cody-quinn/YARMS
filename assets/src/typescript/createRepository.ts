import { query } from "./lib/shorthand";
import { addInputValidator } from "./lib/validate";

import { nameValidator } from "./validators/createRepositoryForm";

// Sections
const typeSection = document.getElementById("type_section");
const typeSpecificSection = document.getElementById("type_specific");

// Form elements
const nameElement = query<HTMLInputElement>("#name");
const nameErrorElement = query<HTMLElement>("#name_error");

const formAllowAnonymousWrites = query<HTMLInputElement>("#allow_anonymous_writes");

const typeGroupChange = (value) => {
  if (value == "Hosted") {
    formAllowAnonymousWrites.removeAttribute("disabled");
    typeSpecificSection.innerHTML = "";
  } else if (value == "Group") {
    formAllowAnonymousWrites.setAttribute("disabled", "true");
    typeSpecificSection.innerHTML = `
<span class="label">Group Members</span>
<input type="text" name="type_group_members" id="type_group_members" placeholder="Group Members (comma seperated)" autocomplete="off">`;
  } else if (value == "Proxy") {
    formAllowAnonymousWrites.setAttribute("disabled", "true");
    typeSpecificSection.innerHTML = ` 
<section>
  <span class="label">Proxy Host</span>
  <input type="text" name="type_proxy_host" id="type_proxy_host" placeholder="Proxy Host" autocomplete="off">
</section>
<section>
  <span class="label">Proxy Cache TTL</span>
  <input type="text" name="type_proxy_cache_ttl" id="type_proxy_cache_ttl" placeholder="Proxy Cache TTL" autocomplete="off">
</section>`;
  }
};

// Validating inputs
addInputValidator(nameElement, nameErrorElement, nameValidator);

// Conditional sections
Array.from(typeSection.getElementsByTagName("input"))
  .filter((el) => el.type == "radio")
  .forEach((el: HTMLInputElement) => {
    el.addEventListener("change", (e) => typeGroupChange(el.value));
    if (el.checked) {
      typeGroupChange(el.value);
    }
  });
