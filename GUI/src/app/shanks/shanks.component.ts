import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {ShanksRequest} from "../models/shanks-request";
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {MatDialog} from "@angular/material/dialog";
import {catchError, EMPTY} from "rxjs";
import {ErrorDialogComponent} from "../error-dialog/error-dialog.component";

@Component({
    selector: "app-shanks",
    standalone: true,
    imports: [CommonModule, FormsModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule],
    templateUrl: "./shanks.component.html",
    styleUrl: "./shanks.component.scss"
})
export class ShanksComponent {

    //Input fields
    public base = "";
    public element = "";
    public modul = "";
    //Output field
    public result = "";

    constructor(private backendRequestService: BackendRequestService, private dialog: MatDialog) {
    }

    /**
     * Berechne die Shanks-Operation.
     */
    public calculate() {
        let body = new ShanksRequest(this.base, this.element, this.modul);
        this.backendRequestService.shanks(body).pipe(
            catchError(
                (error) => {
                    this.dialog.open(ErrorDialogComponent, {
                        data: {message: error.error.message}
                    });
                    return EMPTY;
                }
            )
        ).subscribe(result => {
            this.result = result.message;
        });
    }
}
