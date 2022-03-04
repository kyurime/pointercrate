import {getCountryFlag, getSubdivisionFlag, populateSubdivisionDropdown} from "/static/demonlist/js/modules/demonlist.js";
import {Dropdown, FilteredPaginator, findParentWithClass, get, Viewer} from "/static/core/js/modules/form.js";

export class StatsViewer extends FilteredPaginator {
    /**
     * Constructs a new StatsViewer
     *
     * @param {HTMLElement} html The container element of this stats viewer instance
     * @param statsviewerdata additional settings for this stats viewer
     */
    constructor(html, statsviewerdata) {
        super(
            "stats-viewer-pagination",
            statsviewerdata.entryGenerator,
            "name_contains"
        );

        this.endpoint = statsviewerdata.rankingEndpoint;
        // different from pagination endpoint here!
        this.retrievalEndpoint = statsviewerdata.retrievalEndpoint;
        this.currentLink = this.endpoint + "?" + $.param(this.queryData);

        this.html = html;
        this.output = new Viewer(
            html.getElementsByClassName("viewer-content")[0],
            this
        );

        this._name = document.getElementById("player-name");
        this._created = document.getElementById("created");
        this._beaten = document.getElementById("beaten");
        this._verified = document.getElementById("verified");
        this._published = document.getElementById("published");
        this._hardest = document.getElementById("hardest");
        this._score = document.getElementById("score");
        this._rank = document.getElementById("rank");
        this._amountBeaten = document.getElementById("stats");
        this._welcome = html.getElementsByClassName("viewer-welcome")[0];
        this._progress = document.getElementById("progress");
        this._content = html.getElementsByClassName("viewer-content")[0];

        let dropdownElement = html.getElementsByClassName("dropdown-menu")[0];

        if(dropdownElement !== undefined) {
            this.dropdown = new Dropdown(dropdownElement);
            this.dropdown.addEventListener((selected) => {
                if (selected === "International") {
                    this.updateQueryData("nation", undefined);
                } else {
                    this.updateQueryData("nation", selected);
                }
            });
        }
    }

    initialize() {
        return get("/api/v1/list_information/").then(data => {
            this.list_size = data.data['list_size'];
            this.extended_list_size = data.data['extended_list_size'];

            super.initialize()
        });
    }

    setName(name, nationality) {
        if(nationality === null) {
            this._name.textContent = name;
        } else {
            while (this._name.lastChild) {
                this._name.removeChild(this._name.lastChild);
            }

            let nameSpan = document.createElement("span");
            nameSpan.style.padding = "0 8px";
            nameSpan.innerText = name;

            this._name.appendChild(getCountryFlag(nationality.nation, nationality.country_code));
            this._name.appendChild(nameSpan);

            if (nationality.subdivision !== null) {
                this._name.appendChild(getSubdivisionFlag(nationality.subdivision.name, nationality.country_code, nationality.subdivision.iso_code));
            } else {
                // needed for layout
                this._name.appendChild(document.createElement("span"));
            }
        }
    }

    setHardest(hardest) {
        if(this._hardest.lastChild)
            this._hardest.removeChild(this._hardest.lastChild);
        this._hardest.appendChild(hardest === undefined ? document.createTextNode("None") : this.formatDemon(hardest, "/demonlist/permalink/" + hardest.id + "/"));
    }

    setCompletionNumber(main, extended, legacy) {
        this._amountBeaten.textContent = main + " Main, " + extended + " Extended, " + legacy + " Legacy ";
    }

    onReceive(response) {
        super.onReceive(response);

        // Using currentlySelected is O.K. here, as selection via clicking li-elements is the only possibility (well, not for the nation based one, but oh well)!
        this._rank.innerHTML = this.currentlySelected.dataset.rank;
        this._score.innerHTML = this.currentlySelected.getElementsByTagName(
            "i"
        )[0].innerHTML;
    }

    formatDemon(demon, link) {
        var element;

        if (demon.position <= this.list_size) {
            element = document.createElement("b");
        } else if (demon.position <= this.extended_list_size) {
            element = document.createElement("span");
        } else {
            element = document.createElement("i");
            element.style.opacity = ".5";
        }

        if (link) {
            let a = document.createElement("a");
            a.href = link;
            a.textContent = demon.name;

            element.appendChild(a);
        } else {
            element.textContent = demon.name;
        }

        return element;
    }
}

export function formatInto(parent, childs) {
    while(parent.lastChild) {
        parent.removeChild(parent.lastChild);
    }

    if(childs.length) {
        for(let child of childs) {
            parent.appendChild(child);
            parent.appendChild(document.createTextNode(" - "));
        }

        // remove trailing dash
        parent.removeChild(parent.lastChild);
    } else {
        parent.appendChild(document.createTextNode("None"));
    }
}

export class InteractiveWorldMap {
    constructor() {
        return;
    }

    /**
     * Adds a selection listener to be called when a country/subdivision is selected by clicking
     *
     * @param listener callback (object, object?) -> void taking a nation and optionally a subdivision (both as objects with 'name' and 'code' fields)
     */
    addSelectionListener(listener) {
        return;
    }

    addDeselectionListener(listener) {
        return;
    }

    highlightContinent(continentName) {
        return;
    }

    resetContinentHighlight() {
        return;
    }

    select(nation, subdivision) {
        return;
    }

    deselectSubdivision() {
        return;
    }

    deselect() {
        return;
    }

    showSubdivisions() {
        return;
    }

    hideSubdivisions() {
        return;
    }

    // private

    _select(clicked, fireEvents = true) {
        return;
    }

    _deselect(fireEvents = true) {
        return;
    }

    setLastPosFromTouchEvent(event) {
        return;
    }

    doDrag(deltaX, deltaY) {
        return;
    }

    setupTouchHandlers() {
        return;
    }

    setupMouseHandlers() {
        return;
    }
}
