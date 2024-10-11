package com.ajila.detectors;

import com.ajila.ElementType;

import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.pdmodel.interactive.form.PDField;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * @author fboesiger
 */
public class RepeatableDetector extends Detector {
    public RepeatableDetector() {

    }

    @Override
    public void addElements(PDDocument document, Map<ElementType, Integer> elements) {
        Map<String, Integer> fieldSequence = new HashMap<>();

        for (PDField field : document.getDocumentCatalog().getAcroForm().getFields()) {
            StringBuilder relevantNamePart = new StringBuilder(field.getFullyQualifiedName());
            while (
                    relevantNamePart.length() >= 1 && (
                        isNumber(relevantNamePart.charAt(relevantNamePart.length() - 1))
                        || relevantNamePart.charAt(relevantNamePart.length() - 1) == ' '
                        || relevantNamePart.charAt(relevantNamePart.length() - 1) == '-'
                        || relevantNamePart.charAt(relevantNamePart.length() - 1) == '_')
            ) {
                relevantNamePart.deleteCharAt(relevantNamePart.length() - 1);
            }

            String key = relevantNamePart.toString();

            if (key.isEmpty()) {
                continue;
            }

            fieldSequence.compute(key, (k, v) -> v == null ? 1 : v + 1);
        }

        Map<Integer, List<String>> repeatables = getRepeatables(fieldSequence);

        for (List<String> repeatableContent : repeatables.values()) {
            if (repeatableContent.size() > 1) {
                elements.compute(ElementType.REPEATABLE, (k, v) -> v == null ? 1 : v + 1);
            }
        }
    }

    private static Map<Integer, List<String>> getRepeatables(Map<String, Integer> fieldSequence) {
        Map<Integer, List<String>> numRepetitions = new HashMap<>();

        for (Map.Entry<String, Integer> entry : fieldSequence.entrySet()) {
            if (entry.getValue() > 1) {
                numRepetitions.compute(entry.getValue(), (k, v) -> {
                    if (v == null) {
                        return Collections.singletonList(entry.getKey());
                    } else {
                        List<String> newList = new ArrayList<>(v);
                        newList.add(entry.getKey());
                        return newList;
                    }
                });
            }
        }
        return numRepetitions;
    }

    private static boolean isNumber(char c) {
        return c >= 48 && c < 58;
    }
}
