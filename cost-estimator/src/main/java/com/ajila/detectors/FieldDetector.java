package com.ajila.detectors;

import com.ajila.ElementType;

import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.pdmodel.interactive.form.PDField;

import java.util.Map;

/**
 * @author fboesiger
 */
public class FieldDetector extends Detector {
    private ElementType guessType(PDField field) {
        if (field.getFieldType() != null && field.getFieldType().matches("(?i)Btn|Button")) {
            return ElementType.CHECKBOX;
        }

        return ElementType.TEXTFIELD;
    }

    @Override
    public void addElements(PDDocument document, Map<ElementType, Integer> elements) {
        for (PDField field : document.getDocumentCatalog().getAcroForm().getFields()) {
            elements.compute(guessType(field), (k, v) -> v == null ? 1 : v + 1);
        }
    }
}
