import { queryFormRadiosByName, query } from "./lib/query";
import { addInputValidator } from "./lib/validate";

import { nameValidator } from "./validators/createRepositoryForm";

// Sections
const typeSpecificSection = document.getElementById("type_specific");

// Form elements
const nameElement = query<HTMLInputElement>("#name");
const nameErrorElement = query<HTMLElement>("#name_error");

const formAllowAnonymousWrites = query<HTMLInputElement>("#allow_anonymous_writes");

// Type specific stuff
interface Type {
  name: string;
  form: string;
  onSelect();
}

const repositoryTypeOptions: Type[] = [
  {
    name: "Hosted",
    form: "",
    onSelect() {
      formAllowAnonymousWrites.disabled = false;
    },
  },
  {
    name: "Group",
    form: `
      <span class="label">Group Members</span>
      <input type="text" name="type_group_members" id="type_group_members" placeholder="Group Members (comma seperated)" autocomplete="off">
    `,
    onSelect() {
      formAllowAnonymousWrites.disabled = true;
    },
  },
  {
    name: "Proxy",
    form: `
      <section>
        <span class="label">Proxy Host</span>
        <input type="text" name="type_proxy_host" id="type_proxy_host" placeholder="Proxy Host" autocomplete="off">
      </section>
      <section>
        <span class="label">Proxy Cache TTL</span>
        <input type="text" name="type_proxy_cache_ttl" id="type_proxy_cache_ttl" placeholder="Proxy Cache TTL" autocomplete="off">
      </section>
    `,
    onSelect() {
      formAllowAnonymousWrites.disabled = true;
    },
  },
];

const repositoryTypeChanged = (value) => {
  const newType = repositoryTypeOptions.filter((to) => to.name == value)[0];
  typeSpecificSection.innerHTML = newType.form;
  newType.onSelect();
};

// Validating inputs
addInputValidator(nameElement, nameErrorElement, nameValidator);

query<HTMLFormElement>("#create_repository_form").addEventListener("submit", (ev) => {
  // ev.preventDefault();
  console.dir(ev);
});

// Registering events
queryFormRadiosByName("type").forEach((el) => {
  el.addEventListener("change", (e) => repositoryTypeChanged(el.value));
  if (el.checked) {
    repositoryTypeChanged(el.value);
  }
});
