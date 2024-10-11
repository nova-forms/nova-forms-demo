package com.ajila;

import java.time.Duration;

/**
 * @author fboesiger
 */
public enum ElementType {
    TEXTFIELD(Duration.ZERO, Duration.ZERO, Duration.ofMinutes(15)),
    CHECKBOX(Duration.ZERO, Duration.ZERO, Duration.ofMinutes(10)),
    FILEUPLOAD(Duration.ofHours(12), Duration.ofHours(2), Duration.ZERO),
    SIGNATURE(Duration.ofHours(12), Duration.ofHours(2), Duration.ZERO),
    WORD(Duration.ofHours(12), Duration.ofHours(2), Duration.ofSeconds(5)),
    REPEATABLE(Duration.ZERO, Duration.ZERO, Duration.ofHours(1));

    public final Duration initialSetup;
    public final Duration perDocumentSetup;
    public final Duration perOccurence;

    ElementType(Duration initialSetup, Duration perDocumentSetup, Duration perOccurence) {
        this.initialSetup = initialSetup;
        this.perDocumentSetup = perDocumentSetup;
        this.perOccurence = perOccurence;
    }
}

