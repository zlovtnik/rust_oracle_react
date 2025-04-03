<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import {
        Modal,
        TextInput,
        TextArea,
        DatePicker,
        DatePickerInput,
        Select,
        SelectItem,
    } from "carbon-components-svelte";
    import type { NFeIdentification } from "../types/nfeTypes";

    export let open = false;
    export let initialValues: Partial<NFeIdentification> | undefined =
        undefined;

    let cUF = "";
    let cNF = "";
    let natOp = "";
    let mod_ = "";
    let serie = "";
    let nNF = "";
    let dhEmi: string | undefined = undefined;
    let dhSaiEnt: string | undefined = undefined;
    let tpNF = "";
    let idDest = "";
    let cMunFG = "";
    let tpImp = "";
    let tpEmis = "";
    let cDV = "";
    let tpAmb = "";
    let finNFe = "";
    let indFinal = "";
    let indPres = "";
    let indIntermed = "";
    let procEmi = "";
    let verProc = "";
    let dhCont: string | undefined = undefined;
    let xJust = "";

    const dispatch = createEventDispatcher();

    $: if (open && initialValues) {
        console.log("Form opened with initial values:", initialValues);
        nNF = initialValues.nNF || "";
        serie = initialValues.serie || "";
        dhEmi = initialValues.dhEmi
            ? new Date(initialValues.dhEmi).toISOString().split("T")[0]
            : "";
        natOp = initialValues.natOp || "";
        tpNF = initialValues.tpNF || "";
        cUF = initialValues.cUF || "";
        cNF = initialValues.cNF || "";
        mod_ = initialValues.mod_ || "";
        idDest = initialValues.idDest || "";
        cMunFG = initialValues.cMunFG || "";
        tpImp = initialValues.tpImp || "";
        tpEmis = initialValues.tpEmis || "";
        cDV = initialValues.cDV || "";
        tpAmb = initialValues.tpAmb || "";
        finNFe = initialValues.finNFe || "";
        indFinal = initialValues.indFinal || "";
        indPres = initialValues.indPres || "";
        procEmi = initialValues.procEmi || "";
        verProc = initialValues.verProc || "";
        dhSaiEnt = initialValues.dhSaiEnt || "";
        indIntermed = initialValues.indIntermed || "";
        dhCont = initialValues.dhCont || "";
        xJust = initialValues.xJust || "";
    } else if (open && !initialValues) {
        nNF = "";
        serie = "";
        dhEmi = "";
        natOp = "";
        tpNF = "";
        cUF = "";
        cNF = "";
        mod_ = "";
        idDest = "";
        cMunFG = "";
        tpImp = "";
        tpEmis = "";
        cDV = "";
        tpAmb = "";
        finNFe = "";
        indFinal = "";
        indPres = "";
        procEmi = "";
        verProc = "";
        dhSaiEnt = "";
        indIntermed = "";
        dhCont = "";
        xJust = "";
    }

    async function handleSubmit() {
        console.log("Form submission started");

        // Validate form first
        if (!validateForm()) {
            console.log("Form validation failed");
            return;
        }

        // Format dates to ISO string
        if (!dhEmi) {
            console.error("dhEmi is required but empty");
            alert("Issue Date is required");
            return;
        }

        const formattedDhEmi = new Date(dhEmi + "T00:00:00Z").toISOString();
        const formattedDhSaiEnt = dhSaiEnt
            ? new Date(dhSaiEnt + "T00:00:00Z").toISOString()
            : undefined;
        const formattedDhCont = dhCont
            ? new Date(dhCont + "T00:00:00Z").toISOString()
            : undefined;

        // Create the form data with camelCase field names to match the database
        const formData = {
            cUF: cUF.trim(),
            cNF: cNF.trim(),
            nNF: nNF.trim(),
            tpNF: tpNF.trim(),
            cMunFG: cMunFG.trim(),
            cDV: cDV.trim(),
            finNFe: finNFe.trim(),
            natOp: natOp.trim(),
            mod_: mod_.trim(),
            serie: serie.trim(),
            dhEmi: formattedDhEmi,
            dhSaiEnt: formattedDhSaiEnt,
            idDest: idDest.trim(),
            tpImp: tpImp.trim(),
            tpEmis: tpEmis.trim(),
            tpAmb: tpAmb.trim(),
            indFinal: indFinal.trim(),
            indPres: indPres.trim(),
            indIntermed: indIntermed.trim() || undefined,
            procEmi: procEmi.trim(),
            verProc: verProc.trim(),
            dhCont: formattedDhCont,
            xJust: xJust.trim() || undefined,
        };

        // Log the formatted data being sent
        console.log("Formatted data being sent:", formData);
        console.log(
            "JSON stringified form data:",
            JSON.stringify(formData, null, 2),
        );

        // Check if any required field is null or undefined
        const requiredFields = [
            "cUF",
            "cNF",
            "nNF",
            "tpNF",
            "cMunFG",
            "cDV",
            "finNFe",
            "natOp",
            "mod_",
            "serie",
            "dhEmi",
            "idDest",
            "tpImp",
            "tpEmis",
            "tpAmb",
            "indFinal",
            "indPres",
            "procEmi",
            "verProc",
        ] as const;

        const missingFields = requiredFields.filter(
            (field) => !formData[field as keyof typeof formData],
        );
        if (missingFields.length > 0) {
            console.error("Missing required fields:", missingFields);
            alert(`Missing required fields: ${missingFields.join(", ")}`);
            return;
        }

        // Ensure cUF is not empty
        if (!formData.cUF) {
            console.error("cUF is empty");
            alert("UF Code is required");
            return;
        }

        console.log("Dispatching form data");
        dispatch("finish", formData);
    }

    function handleCancel() {
        console.log("Form cancelled");
        dispatch("cancel");
    }

    function validateForm() {
        console.log("Validating form");
        // Validate all required fields
        if (
            !cUF ||
            !cNF ||
            !natOp ||
            !mod_ ||
            !serie ||
            !nNF ||
            !dhEmi ||
            !tpNF ||
            !idDest ||
            !cMunFG ||
            !tpImp ||
            !tpEmis ||
            !cDV ||
            !tpAmb ||
            !finNFe ||
            !indFinal ||
            !indPres ||
            !procEmi ||
            !verProc
        ) {
            console.log("Form validation failed - missing required fields");
            alert("Please fill in all required fields");
            return false;
        }
        console.log("Form validation passed");
        return true;
    }
</script>

<Modal
    {open}
    modalHeading="Add NFe Identification"
    primaryButtonText="Create"
    secondaryButtonText="Cancel"
    on:click:button--primary={handleSubmit}
    on:click:button--secondary={handleCancel}
    on:close={handleCancel}
    class="custom-modal"
>
    <div class="form-container">
        <TextInput
            labelText="UF Code"
            placeholder="Enter UF code"
            bind:value={cUF}
            required
        />
        <TextInput
            labelText="NFe Code"
            placeholder="Enter NFe code"
            bind:value={cNF}
            required
        />
        <TextInput
            labelText="Model"
            placeholder="Enter model"
            bind:value={mod_}
            required
        />
        <TextInput
            labelText="Series"
            placeholder="Enter series"
            bind:value={serie}
            required
        />
        <TextInput
            labelText="NFe Number"
            placeholder="Enter NFe number"
            bind:value={nNF}
            required
        />
        <DatePicker
            dateFormat="Y-m-d"
            datePickerType="single"
            bind:value={dhEmi}
        >
            <DatePickerInput
                placeholder="YYYY-MM-DD"
                labelText="Issue Date"
                required
            />
        </DatePicker>
        <TextInput
            labelText="Nature of Operation"
            placeholder="Enter nature of operation"
            bind:value={natOp}
            required
        />
        <div class="bx--form-item">
            <label class="bx--label">Type of NFe</label>
            <select class="bx--select-input" bind:value={tpNF} required>
                <option value="">Select a type</option>
                <option value="0">Entry (0)</option>
                <option value="1">Exit (1)</option>
            </select>
        </div>
        <TextInput
            labelText="Destination ID"
            placeholder="Enter destination ID"
            bind:value={idDest}
            required
        />
        <TextInput
            labelText="Municipality Code"
            placeholder="Enter municipality code"
            bind:value={cMunFG}
            required
        />
        <TextInput
            labelText="Print Type"
            placeholder="Enter print type"
            bind:value={tpImp}
            required
        />
        <TextInput
            labelText="Emission Type"
            placeholder="Enter emission type"
            bind:value={tpEmis}
            required
        />
        <TextInput
            labelText="Check Digit"
            placeholder="Enter check digit"
            bind:value={cDV}
            required
        />
        <TextInput
            labelText="Environment Type"
            placeholder="Enter environment type"
            bind:value={tpAmb}
            required
        />
        <TextInput
            labelText="NFe Finality"
            placeholder="Enter NFe finality"
            bind:value={finNFe}
            required
        />
        <TextInput
            labelText="Final Consumer Indicator"
            placeholder="Enter final consumer indicator"
            bind:value={indFinal}
            required
        />
        <TextInput
            labelText="Presence Indicator"
            placeholder="Enter presence indicator"
            bind:value={indPres}
            required
        />
        <TextInput
            labelText="Emission Process"
            placeholder="Enter emission process"
            bind:value={procEmi}
            required
        />
        <TextInput
            labelText="Process Version"
            placeholder="Enter process version"
            bind:value={verProc}
            required
        />
        <DatePicker
            dateFormat="Y-m-d"
            datePickerType="single"
            bind:value={dhSaiEnt}
        >
            <DatePickerInput
                placeholder="YYYY-MM-DD"
                labelText="Sai Date"
                required
            />
        </DatePicker>
        <TextInput
            labelText="Intermediate Consumer Indicator"
            placeholder="Enter intermediate consumer indicator"
            bind:value={indIntermed}
            required
        />
        <DatePicker
            dateFormat="Y-m-d"
            datePickerType="single"
            bind:value={dhCont}
        >
            <DatePickerInput
                placeholder="YYYY-MM-DD"
                labelText="Cont Date"
                required
            />
        </DatePicker>
        <TextInput
            labelText="Justification"
            placeholder="Enter justification"
            bind:value={xJust}
            required
        />
    </div>
</Modal>

<style>
    .form-container {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 1rem;
        padding: 1rem;
    }

    :global(.custom-modal .bx--modal-header) {
        background-color: var(--td-green);
        padding: 1.5rem;
    }

    :global(.custom-modal .bx--modal-header__heading) {
        color: white;
        font-size: 1.25rem;
    }

    :global(.custom-modal .bx--modal-content) {
        padding: 1.5rem;
        background-color: var(--dark-bg-light);
    }

    :global(.custom-modal .bx--modal-container) {
        background-color: var(--dark-bg-light);
    }

    :global(.custom-modal .bx--btn--primary) {
        background-color: var(--violet);
    }

    :global(.custom-modal .bx--btn--primary:hover) {
        background-color: var(--violet-light);
    }

    :global(.custom-modal .bx--label) {
        color: var(--light-gray);
    }

    :global(
            .custom-modal .bx--text-input,
            .custom-modal .bx--text-area,
            .custom-modal .bx--select-input
        ) {
        background-color: var(--dark-bg);
        color: var(--white);
        border-bottom-color: var(--mid-gray);
    }

    :global(
            .custom-modal .bx--text-input:focus,
            .custom-modal .bx--text-area:focus,
            .custom-modal .bx--select-input:focus
        ) {
        border-bottom-color: var(--violet);
    }

    :global(.custom-modal .bx--select-input__wrapper::before) {
        background-color: var(--violet);
    }
</style>
